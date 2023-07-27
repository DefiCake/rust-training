mod wallet;

use fuel_core::{
  database::Database,
  chain_config::{ ChainConfig, StateConfig, CoinConfig, ChainConfigDb },
  service::{ Config as FuelServiceConfig, FuelService },
  types::{
    fuel_types::{ Address, AssetId, Nonce },
    fuel_crypto::rand::{ rngs::StdRng, Rng, RngCore, SeedableRng },
    blockchain::primitives::DaBlockHeight,
    entities::message::Message,
    fuel_vm::SecretKey,
  },
  executor::RelayerPort,
};
use fuels::{
  prelude::{ WalletUnlocked, Provider, Account, Signer },
  accounts::wallet::Wallet as FuelsViewWallet,
  types::transaction_builders::{ ScriptTransactionBuilder, TransactionBuilder },
};
use wallet::wallet::Wallet;

#[derive(Clone, Debug)]
struct MyRelayer {
  database: Database,
}

impl RelayerPort for MyRelayer {
  fn get_message(&self, id: &Nonce, _da_height: &DaBlockHeight) -> anyhow::Result<Option<Message>> {
    use fuel_core_storage::{ tables::Messages, StorageAsRef };
    use std::borrow::Cow;
    Ok(self.database.storage::<Messages>().get(id)?.map(Cow::into_owned))
  }
}

#[tokio::main]
async fn main() {
  env_logger::init();

  let mut rng = StdRng::seed_from_u64(10);
  // a coin with all options set
  let alice_secret: SecretKey = rng.gen();
  let alice = Wallet::new(alice_secret);
  let asset_id_alice: AssetId = rng.gen();
  let alice_value = rng.gen();
  let alice_maturity = Some(rng.next_u32().into());
  let alice_block_created = Some(rng.next_u32().into());
  let alice_block_created_tx_idx = Some(rng.gen());
  let alice_tx_id = Some(rng.gen());
  let alice_output_index = Some(rng.gen());
  // let alice_utxo_id = UtxoId::new(alice_tx_id.unwrap(), alice_output_index.unwrap());

  // a coin with minimal options set
  let bob: Address = rng.gen();
  let asset_id_bob: AssetId = rng.gen();
  let bob_value = rng.gen();

  let fuel_service_config = FuelServiceConfig {
    chain_conf: ChainConfig {
      initial_state: Some(StateConfig {
        coins: Some(
          vec![
            CoinConfig {
              tx_id: alice_tx_id,
              output_index: alice_output_index,
              tx_pointer_block_height: alice_block_created,
              tx_pointer_tx_idx: alice_block_created_tx_idx,
              maturity: alice_maturity,
              owner: alice.clone().into(),
              amount: alice_value,
              asset_id: asset_id_alice,
            },
            CoinConfig {
              tx_id: None,
              output_index: None,
              tx_pointer_block_height: None,
              tx_pointer_tx_idx: None,
              maturity: None,
              owner: bob,
              amount: bob_value,
              asset_id: asset_id_bob,
            }
          ]
        ),
        height: alice_block_created.map(|h| {
          let mut h: u32 = h.into();
          // set starting height to something higher than alice's coin
          h = h.saturating_add(rng.next_u32());
          h.into()
        }),
        ..Default::default()
      }),
      ..ChainConfig::local_testnet()
    },
    ..FuelServiceConfig::local_node()
  };

  let srv = FuelService::from_database(Database::in_memory(), fuel_service_config.clone()).await.unwrap();
  srv.await_relayer_synced().await.unwrap();

  let provider = Provider::connect(srv.bound_address.to_string()).await.unwrap();
  let w = WalletUnlocked::new_from_private_key(alice_secret, Some(provider.clone()));
  let t = FuelsViewWallet::from_address(bob.into(), None);
  let mut inputs = vec![];
  let i = w.get_asset_inputs_for_amount(asset_id_alice, alice_value / 2, None).await.unwrap();
  inputs.extend(i);

  let mut outputs = vec![];
  let o = w.get_asset_outputs_for_amount(t.address(), asset_id_alice, alice_value / 2);
  outputs.extend(o);

  let mut tx = ScriptTransactionBuilder::prepare_transfer(inputs, outputs, Default::default())
    // .set_gas_limit(fuel_service_config.chain_conf.block_gas_limit / 2)
    .build()
    .unwrap();

  w.sign_transaction(&mut tx).unwrap();

  let receipt = provider.send_transaction(&tx).await.unwrap();
  println!("receipt {:?}", receipt);
}

// let tx = TransactionBuilder::script(op::ret(RegId::ONE).to_bytes().into_iter().collect(), vec![])
//   .add_unsigned_coin_input(
//     alice.clone().into(),
//     utxo_id,
//     alice_value,
//     Default::default(),
//     Default::default(),
//     Default::default()
//   )
//   .gas_limit(fuel_service_config.chain_conf.block_gas_limit - 1)
//   .gas_price(1)
//   .add_output(Output::Change { to: bob, amount: alice_value, asset_id: Default::default() })
//   .finalize_as_transaction();

// let result = srv.submit_and_await_commit(tx).await.unwrap();

// println!("Transaction: {:?}", result);

// let coin = client.coin(&utxo_id).await.unwrap();
// let latestBlock = database.latest_block().unwrap();

// let executor: Executor<MyRelayer> = Executor {
//   relayer,
//   database,
//   config: Arc::new(Default::default()),
// };

// let block: Block = Block::new(latestBlock.header().into(), [].into(), &[]);

// let executionOptions: ExecutionOptions = Default::default();
// executor.execute_without_commit(block, executionOptions);

// println!("coin: {:?}", coin);
// println!("owner: {:?}", alice);
