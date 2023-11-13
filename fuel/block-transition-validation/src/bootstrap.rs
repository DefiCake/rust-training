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
use fuel_tx::Script;
use fuels::{
  prelude::{ WalletUnlocked, Provider, Account, Signer },
  accounts::wallet::Wallet as FuelsViewWallet,
  types::transaction_builders::{ ScriptTransactionBuilder, TransactionBuilder },
  tx::Bytes32,
};

use crate::{ wallet::wallet::Wallet, cli::cli::DBType, dbsnap::dbsnap::snapshot };
use crate::serialization::lib::BinFileSerde;

pub async fn bootstrap(db_type: DBType) {
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

  let database = match db_type {
    DBType::Memory => { Database::in_memory() }
    DBType::Rocks => {
      let datasource = Arc::new(RocksDb::default_open(path, None).unwrap());
      Database::new(datasource)
    }
  };

  let srv = FuelService::from_database(database.clone(), fuel_service_config.clone()).await.unwrap();
  srv.await_relayer_synced().await.unwrap();

  snapshot(srv.shared.database.clone(), "snapshot_a.json".into()).expect("Failed to do first snapshot");

  let provider = Provider::connect(srv.bound_address.to_string()).await.unwrap();

  let block_a = srv.shared.database.get_current_block().unwrap().unwrap();

  let w = WalletUnlocked::new_from_private_key(alice_secret, Some(provider.clone()));
  let t = FuelsViewWallet::from_address(bob.into(), None);

  let mut inputs = vec![];
  let i = w.get_asset_inputs_for_amount(asset_id_alice, alice_value / 2).await.unwrap();
  inputs.extend(i);

  let mut outputs = vec![];
  let o = w.get_asset_outputs_for_amount(t.address(), asset_id_alice, alice_value / 2);
  outputs.extend(o);

  let network_info = provider.network_info().await.unwrap();

  
  let mut tb = 
    ScriptTransactionBuilder::prepare_transfer(inputs, outputs, Default::default(), network_info.clone());
  w.sign_transaction(&mut tb);

  let tx = tb.build().unwrap();

  Into::<Script>
    ::into(tx.clone())
    .to_bincode_file("transaction.bincode".into())
    .expect("Error serializing transaction");

  let tx_id = provider.send_transaction(tx).await.unwrap();
  dbg!(tx_id); 
  let pb = indicatif::ProgressBar::new_spinner();
  pb.enable_steady_tick(std::time::Duration::from_millis(120));
  pb.set_style(
      indicatif::ProgressStyle::with_template("{spinner:.blue} {msg}")
          .unwrap()
          .tick_strings(&[
              "▹▹▹▹▹",
              "▸▹▹▹▹",
              "▹▸▹▹▹",
              "▹▹▸▹▹",
              "▹▹▹▸▹",
              "▹▹▹▹▸",
              "▪▪▪▪▪",
          ]),
  );
  pb.set_message("Waiting for receipt...");

  loop {
    let receipts = provider.tx_status(&tx_id).await;

    if receipts.is_ok() {
      break;
    }
  }

  pb.finish_with_message("Waiting for receipt... Finished");

  let receipts = provider.tx_status(&tx_id).await.unwrap().take_receipts();

  dbg!(receipts);

  // let alice_balance = provider.get_asset_balance(&alice.clone().into(), Default::default()).await.unwrap();
  // let bob_balance = provider.get_asset_balance(&bob.clone().into(), Default::default()).await.unwrap();
  // dbg!(alice_balance);
  // dbg!(bob_balance);

  let block_b = srv.shared.database.get_current_block().unwrap().unwrap();
  // This does not get me enough information to rebuild the block and block transition...
  // to_json_file(&block_a, "block_a.json".to_string()).expect("Failed block_a json write");
  // to_json_file(&block_b, "block_b.json".to_string()).expect("Failed block_b write");

  block_a.to_bincode_file("block_a.bincode".to_string()).expect("Failed block_a bincode write");
  block_b.to_bincode_file("block_b.bincode".to_string()).expect("Failed block_a bincode write");
  snapshot(srv.shared.database.clone(), "snapshot_b.json".into()).expect("Failed to do second snapshot");

  let read_block_b: Block<Bytes32> = BinFileSerde::from_bincode_file("block_b.bincode".to_string()).expect(
    "Failed to roundtrip block_b.bincode"
  );

  assert_eq!(read_block_b, block_b.clone().into_owned());
  dbg!(block_b.header());
  dbg!(block_b.header().hash());
}
