use std::sync::Arc;

use fuel_core::{
  database::Database,
  executor::{ RelayerPort, Executor },
  types::{ blockchain::{ primitives::DaBlockHeight, block::Block }, entities::message::Message },
};
use fuels::{ types::Nonce, tx::Bytes32 };

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

pub async fn load() {
  //   let start_block: Block<Bytes32> = from_bincode_file("block_a.bincode".to_string()).expect("Could not load start_block");
  let _start_block: Block<Bytes32> = BinFileSerde::from_bincode_file("block_a.bincode".to_string()).expect(
    "Could not load start_block"
  );

  let database = Database::in_memory();
  //   database.init(config);
  let relayer: MockRelayer = MockRelayer { database: database.clone() };
  let _executor: Executor<MockRelayer> = Executor {
    relayer,
    database,
    config: Arc::new(Default::default()),
  };
}
