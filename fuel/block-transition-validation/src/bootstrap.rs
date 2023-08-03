use std::{ path::Path, sync::Arc };

use fuel_core::{
  database::Database,
  chain_config::{ ChainConfig, StateConfig, CoinConfig },
  service::{ Config as FuelServiceConfig, FuelService },
  types::{
    fuel_types::{ Address, AssetId },
    fuel_crypto::rand::{ rngs::StdRng, Rng, SeedableRng },
    fuel_vm::SecretKey,
    blockchain::block::Block,
  },
  state::rocks_db::RocksDb,
};
use fuels::{
  prelude::{ WalletUnlocked, Provider, Account, Signer },
  accounts::wallet::Wallet as FuelsViewWallet,
  types::transaction_builders::{ ScriptTransactionBuilder, TransactionBuilder },
  tx::Bytes32,
};

use crate::{ wallet::wallet::Wallet, memstore::MemStore };
use crate::serialization::lib::BinFileSerde;

pub async fn bootstrap() {
  let path_string = String::from("rocksdb");
  let path = Path::new(&path_string);
  let _res = std::fs::remove_dir_all(&path_string);

  let mut rng = StdRng::seed_from_u64(10);
  // a coin with all options set
  let alice_secret: SecretKey = rng.gen();
  let alice = Wallet::new(alice_secret);
  let asset_id_alice: AssetId = Default::default();
  let alice_value = 10_000_000;
  let alice_maturity = Some((0).into());
  let alice_block_created = Some((0).into());
  let alice_block_created_tx_idx = Some(0);
  let alice_tx_id = Some(rng.gen());
  let alice_output_index = Some(rng.gen());

  let bob: Address = rng.gen();

  let fuel_service_config = FuelServiceConfig {
    chain_conf: ChainConfig {
      initial_state: Some(StateConfig {
        coins: Some(
          vec![CoinConfig {
            tx_id: alice_tx_id,
            output_index: alice_output_index,
            tx_pointer_block_height: alice_block_created,
            tx_pointer_tx_idx: alice_block_created_tx_idx,
            maturity: alice_maturity,
            owner: alice.clone().into(),
            amount: alice_value,
            asset_id: asset_id_alice,
          }]
        ),
        height: Some((0).into()),
        ..Default::default()
      }),
      ..ChainConfig::local_testnet()
    },
    ..FuelServiceConfig::local_node()
  };

  let datasource = Arc::new(MemStore::default());
  // let datasource = Arc::new(RocksDb::default_open(path, None).unwrap());

  let database = Database::new(datasource);
  let srv = FuelService::from_database(database.clone(), fuel_service_config.clone()).await.unwrap();
  srv.await_relayer_synced().await.unwrap();

  let provider = Provider::connect(srv.bound_address.to_string()).await.unwrap();

  let block_a = srv.shared.database.get_current_block().unwrap().unwrap();

  let w = WalletUnlocked::new_from_private_key(alice_secret, Some(provider.clone()));
  let t = FuelsViewWallet::from_address(bob.into(), None);

  let mut inputs = vec![];
  let i = w.get_asset_inputs_for_amount(asset_id_alice, alice_value / 2, None).await.unwrap();
  inputs.extend(i);

  let mut outputs = vec![];
  let o = w.get_asset_outputs_for_amount(t.address(), asset_id_alice, alice_value / 2);
  outputs.extend(o);

  let mut tx = ScriptTransactionBuilder::prepare_transfer(inputs, outputs, Default::default()).build().unwrap();

  w.sign_transaction(&mut tx).unwrap();

  let receipt = provider.send_transaction(&tx).await.unwrap();
  println!("receipt {:?}", receipt);

  let alice_balance = provider.get_asset_balance(&alice.clone().into(), Default::default()).await.unwrap();
  let bob_balance = provider.get_asset_balance(&bob.clone().into(), Default::default()).await.unwrap();
  dbg!(alice_balance);
  dbg!(bob_balance);

  let block_b = srv.shared.database.get_current_block().unwrap().unwrap();

  // This does not get me enough information to rebuild the block and block transition...
  // to_json_file(&block_a, "block_a.json".to_string()).expect("Failed block_a json write");
  // to_json_file(&block_b, "block_b.json".to_string()).expect("Failed block_b write");

  block_a.to_bincode_file("block_a.bincode".to_string()).expect("Failed block_a bincode write");
  block_b.to_bincode_file("block_b.bincode".to_string()).expect("Failed block_a bincode write");

  let read_block_b: Block<Bytes32> = BinFileSerde::from_bincode_file("block_b.bincode".to_string()).expect("a");

  assert_eq!(read_block_b, block_b.into_owned());
}
