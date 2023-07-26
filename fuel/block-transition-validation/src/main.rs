use std::sync::Arc;

use fuel_core::{
  database::{ transaction::DatabaseTransaction, transactions::TransactionIndex, vm_database::VmDatabase, Database },
  chain_config::{ ChainConfig, StateConfig, CoinConfig },
  service::{ Config, FuelService },
  types::{
    fuel_types::{ Address, AssetId, Nonce },
    fuel_crypto::rand::{ rngs::StdRng, Rng, RngCore, SeedableRng },
    blockchain::primitives::DaBlockHeight,
    entities::message::Message,
  },
  executor::{ Executor, RelayerPort },
};
use fuels::client::FuelClient;

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
  let alice: Address = rng.gen();
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

  let service_config = Config {
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
              owner: alice,
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
    ..Config::local_node()
  };

  let database = Database::in_memory();
  let relayer: MyRelayer = MyRelayer { database: database.clone() };
  let srv = FuelService::from_database(database.clone(), service_config).await.unwrap();
  let client = FuelClient::from(srv.bound_address);
  srv.await_relayer_synced().await.unwrap();

  let alice_tx_id_bytes: [u8; 32] = alice_tx_id.unwrap().into();
  let convertedUtxoId: fuels::types::UtxoId = fuels::types::UtxoId::new(
    fuels::tx::Bytes32::from(alice_tx_id_bytes),
    alice_output_index.unwrap()
  );
  let coin = client.coin(&convertedUtxoId).await.unwrap();

  let executor: Executor<MyRelayer> = Executor {
    relayer,
    database,
    config: Arc::new(Default::default()),
  };

  // Next step is to execute a block
  // executor.execute_without_commit(block, options)

  println!("coin: {:?}", coin);
  println!("owner: {:?}", alice);
}
