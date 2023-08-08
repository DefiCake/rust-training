use std::sync::Arc;

use fuel_core::{
  database::Database,
  executor::{ RelayerPort, Executor },
  types::{ blockchain::{ primitives::DaBlockHeight, block::Block }, entities::message::Message },
  chain_config::ChainConfig,
};
use fuel_tx::{ Script, Transaction };
use fuels::types::Nonce;

use crate::serialization::lib::BinFileSerde;

#[derive(Clone, Debug)]
struct MockRelayer {
  database: Database,
}

impl RelayerPort for MockRelayer {
  fn get_message(&self, id: &Nonce, _da_height: &DaBlockHeight) -> anyhow::Result<Option<Message>> {
    use fuel_core_storage::{ tables::Messages, StorageAsRef };
    use std::borrow::Cow;
    Ok(self.database.storage::<Messages>().get(id)?.map(Cow::into_owned))
  }
}

pub async fn load() -> anyhow::Result<()> {
  let json = std::fs::read_to_string("snapshot_a.json".to_string())?;
  let config: ChainConfig = serde_json::from_str(json.as_str())?;

  let database = Database::in_memory();
  database.init(&config)?;

  let relayer: MockRelayer = MockRelayer { database: database.clone() };
  let _executor: Executor<MockRelayer> = Executor {
    relayer,
    database,
    config: Arc::new(Default::default()),
  };

  let script = Script::from_bincode_file("transaction.bincode".into())?;
  let transaction = Into::<Transaction>::into(script);
  let mut block: Block<Transaction> = Block::default();
  *block.transactions_mut() = [transaction].into();

  Ok(())
}

// use std::{ path::Path, sync::Arc };

// use fuel_core::{
//   database::Database,
//   chain_config::{ ChainConfig, StateConfig, CoinConfig },
//   service::{ Config as FuelServiceConfig, FuelService },
//   types::{
//     fuel_types::{ Address, AssetId },
//     fuel_crypto::rand::{ rngs::StdRng, Rng, SeedableRng },
//     fuel_vm::SecretKey,
//     blockchain::block::Block,
//   },
//   state::rocks_db::RocksDb,
//   executor::{ Executor, RelayerPort },
// };
// use fuel_core_types::{
//   fuel_types::Nonce,
//   blockchain::primitives::DaBlockHeight,
//   entities::message::Message,
//   services::executor::ExecutionBlock,
// };
// use fuel_tx::{ Script, Transaction };
// use fuels::{
//   prelude::{ WalletUnlocked, Provider, Account, Signer },
//   accounts::wallet::Wallet as FuelsViewWallet,
//   types::transaction_builders::{ ScriptTransactionBuilder, TransactionBuilder },
//   tx::Bytes32,
// };

// use crate::{ wallet::wallet::Wallet, cli::cli::DBType, dbsnap::dbsnap::snapshot };
// use crate::serialization::lib::BinFileSerde;

// pub async fn bootstrap(db_type: DBType) {
//   let path_string = String::from("rocksdb");
//   let path = Path::new(&path_string);
//   let _res = std::fs::remove_dir_all(&path_string);

//   let mut rng = StdRng::seed_from_u64(10);
//   // a coin with all options set
//   let alice_secret: SecretKey = rng.gen();
//   let alice = Wallet::new(alice_secret);
//   let asset_id_alice: AssetId = Default::default();
//   let alice_value = 10_000_000;
//   let alice_maturity = Some((0).into());
//   let alice_block_created = Some((0).into());
//   let alice_block_created_tx_idx = Some(0);
//   let alice_tx_id = Some(rng.gen());
//   let alice_output_index = Some(rng.gen());

//   let bob: Address = rng.gen();

//   let fuel_service_config = FuelServiceConfig {
//     chain_conf: ChainConfig {
//       initial_state: Some(StateConfig {
//         coins: Some(
//           vec![CoinConfig {
//             tx_id: alice_tx_id,
//             output_index: alice_output_index,
//             tx_pointer_block_height: alice_block_created,
//             tx_pointer_tx_idx: alice_block_created_tx_idx,
//             maturity: alice_maturity,
//             owner: alice.clone().into(),
//             amount: alice_value,
//             asset_id: asset_id_alice,
//           }]
//         ),
//         height: Some((0).into()),
//         ..Default::default()
//       }),
//       ..ChainConfig::local_testnet()
//     },
//     ..FuelServiceConfig::local_node()
//   };

//   let database = match db_type {
//     DBType::Memory => { Database::in_memory() }
//     DBType::Rocks => {
//       let datasource = Arc::new(RocksDb::default_open(path, None).unwrap());
//       Database::new(datasource)
//     }
//   };

//   // let srv = FuelService::from_database(database.clone(), fuel_service_config.clone()).await.unwrap();
//   // srv.await_relayer_synced().await.unwrap();

//   // snapshot(srv.shared.database.clone(), "snapshot_a.json".into()).expect("Failed to do first snapshot");

//   // let provider = Provider::connect(srv.bound_address.to_string()).await.unwrap();

//   // let block_a = srv.shared.database.get_current_block().unwrap().unwrap();

//   let w = WalletUnlocked::new_from_private_key(alice_secret, None);
//   let t = FuelsViewWallet::from_address(bob.into(), None);

//   let mut inputs = vec![];
//   let i = w.get_asset_inputs_for_amount(asset_id_alice, alice_value / 2, None).await.unwrap();
//   inputs.extend(i);

//   let mut outputs = vec![];
//   let o = w.get_asset_outputs_for_amount(t.address(), asset_id_alice, alice_value / 2);
//   outputs.extend(o);

//   let mut signedTx = ScriptTransactionBuilder::prepare_transfer(inputs, outputs, Default::default()).build().unwrap();
//   w.sign_transaction(&mut signedTx).unwrap();

//   let transaction: Transaction = signedTx.clone().into();

// let mut block: Block<Transaction> = Block::default();
// *block.transactions_mut() = [transaction].into();

//   let executor: Executor<MyRelayer> = Executor {
//     relayer: MyRelayer { database: database.clone() },
//     database,
//     config: Arc::new(Default::default()),
//   };

//   executor
//     .execute_without_commit(ExecutionBlock::Production(block.into()), Default::default())
//     .expect("Could not commit");

//   // Into::<Script>
//   //   ::into(signedTx.clone())
//   //   .to_bincode_file("transaction.bincode".into())
//   //   .expect("Error serializing transaction");

//   // let receipt = provider.send_transaction(&tx).await.unwrap();
//   // println!("receipt {:?}", receipt);

//   // let alice_balance = provider.get_asset_balance(&alice.clone().into(), Default::default()).await.unwrap();
//   // let bob_balance = provider.get_asset_balance(&bob.clone().into(), Default::default()).await.unwrap();
//   // dbg!(alice_balance);
//   // dbg!(bob_balance);

//   // let block_b = srv.shared.database.get_current_block().unwrap().unwrap();

//   // // This does not get me enough information to rebuild the block and block transition...
//   // // to_json_file(&block_a, "block_a.json".to_string()).expect("Failed block_a json write");
//   // // to_json_file(&block_b, "block_b.json".to_string()).expect("Failed block_b write");

//   // block_a.to_bincode_file("block_a.bincode".to_string()).expect("Failed block_a bincode write");
//   // block_b.to_bincode_file("block_b.bincode".to_string()).expect("Failed block_a bincode write");
//   // snapshot(srv.shared.database.clone(), "snapshot_b.json".into()).expect("Failed to do second snapshot");

//   // let read_block_b: Block<Bytes32> = BinFileSerde::from_bincode_file("block_b.bincode".to_string()).expect("a");

//   // assert_eq!(read_block_b, block_b.into_owned());
// }

// #[derive(Clone, Debug, Default)]
// struct MyRelayer {
//   database: Database,
// }

// impl RelayerPort for MyRelayer {
//   fn get_message(&self, id: &Nonce, _da_height: &DaBlockHeight) -> anyhow::Result<Option<Message>> {
//     use fuel_core_storage::{ tables::Messages, StorageAsRef };
//     use std::borrow::Cow;
//     Ok(self.database.storage::<Messages>().get(id)?.map(Cow::into_owned))
//   }
// }
