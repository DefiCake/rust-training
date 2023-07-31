mod wallet;
mod serialization;
mod cli;

use fuel_core::{
  database::Database,
  chain_config::{ ChainConfig, StateConfig, CoinConfig },
  service::{ Config as FuelServiceConfig, FuelService },
  types::{
    fuel_types::{ Address, AssetId },
    fuel_crypto::rand::{ rngs::StdRng, Rng, RngCore, SeedableRng },
    fuel_vm::SecretKey,
  },
};
use fuels::{
  prelude::{ WalletUnlocked, Provider, Account, Signer },
  accounts::wallet::Wallet as FuelsViewWallet,
  types::transaction_builders::{ ScriptTransactionBuilder, TransactionBuilder },
};
use wallet::wallet::Wallet;
use serialization::{ json::to_json_file, bincode::{ from_bincode_file, to_bincode_file } };

use cli::cli::{ Mode, mode };

#[tokio::main]
async fn main() {
  env_logger::init();

  match mode() {
    Mode::Bootstrap => {
      println!("BOOTSTRAP");
    }
    Mode::Load => {
      println!("LOAD");
    }
  }

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

  let database = Database::in_memory();
  let srv = FuelService::from_database(database.clone(), fuel_service_config.clone()).await.unwrap();
  srv.await_relayer_synced().await.unwrap();

  let provider = Provider::connect(srv.bound_address.to_string()).await.unwrap();

  let block_a = srv.shared.database.get_current_block().unwrap().unwrap();
  // println!("Block A: {:?}", block_a);

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

  let block_b = srv.shared.database.get_current_block().unwrap().unwrap();
  // println!("Block B: {:?}", block_b);

  // This does not get me enough information to rebuild the block and block transition...
  to_json_file(&block_a, "block_a.json".to_string()).expect("Failed block_a json write");
  to_json_file(&block_b, "block_b.json".to_string()).expect("Failed block_b write");

  to_bincode_file(&block_a, "block_a.bincode".to_string()).expect("Failed block_a bincode write");
  to_bincode_file(&block_b, "block_b.bincode".to_string()).expect("Failed block_a bincode write");

  let read_block_b = from_bincode_file("block_b.bincode".to_string()).expect("Failed block_a bincode read");

  // println!("Read block b");
  // println!("{:?}", &read_block_b);
  // println!("{:?}", &block_b);
  assert_eq!(read_block_b, block_b.into_owned());
}
