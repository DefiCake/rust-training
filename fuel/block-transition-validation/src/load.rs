use std::sync::Arc;

use fuel_core::{
  database::Database,
  executor::{ RelayerPort, Executor, OnceTransactionsSource },
  types::{ blockchain::{ primitives::DaBlockHeight, block::Block }, entities::message::Message },
  chain_config::ChainConfig,
};
use fuel_core_types::{ services::{executor::ExecutionTypes, block_producer::Components}, blockchain::header::PartialBlockHeader };
use fuel_tx::{ Script, Transaction };
use fuels::types::Nonce;
use fuels::tx::Bytes32;

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

pub fn load() -> anyhow::Result<()> {
  let json = std::fs::read_to_string("snapshot_a.json".to_string())?;
  let config: ChainConfig = serde_json::from_str(json.as_str())?;
  let initial_state = config.clone().initial_state.expect("Could not load initial state");
  let initial_height = initial_state.height.expect("Could not load initial height");
  let database = Database::in_memory();
  database.init(&config)?;

  let relayer: MockRelayer = MockRelayer { database: database.clone() };
  let executor: Executor<MockRelayer> = Executor {
    relayer,
    database: database.clone(),
    config: Arc::new(Default::default()),
  };

  let block: Block<Bytes32> = BinFileSerde::from_bincode_file("block_b.bincode".into()).expect(
    "Could not deserialize block"
  );
  let time = block.header().time();

  let height: fuel_crypto::fuel_types::BlockHeight = (u32::from(initial_height) + 1u32).into();
  let prev_root = block.header().prev_root().clone();

  let script = Script::from_bincode_file("transaction.bincode".into())?;
  let transaction = Into::<Transaction>::into(script);

  let mut def = PartialBlockHeader::default();
  def.consensus.prev_root = prev_root;
  def.consensus.time = time;
  def.consensus.height = height;

  let reproduced_block_header: PartialBlockHeader = PartialBlockHeader { ..def };

  let component = ExecutionTypes::Production(Components {
    header_to_produce: reproduced_block_header,
    transactions_source: OnceTransactionsSource::new([transaction].into()),
    gas_limit: u64::MAX
  });
  

//   let component = match block {
//     ExecutionTypes::DryRun(_) => {
//         panic!("It is not possible to commit the dry run result");
//     }
//     ExecutionTypes::Production(block) => ExecutionTypes::Production(Components {
//         header_to_produce: block.header,
//         transactions_source: OnceTransactionsSource::new(block.transactions),
//         gas_limit: u64::MAX,
//     }),
//     ExecutionTypes::Validation(block) => ExecutionTypes::Validation(block),
// };

// let (result, db_transaction) =
//     self.execute_without_commit(component, options)?.into();

  let execution_result = executor.execute_without_commit(
    component,
    Default::default()
  )?;

  dbg!(execution_result.result().block.header());

  Ok(())
}
