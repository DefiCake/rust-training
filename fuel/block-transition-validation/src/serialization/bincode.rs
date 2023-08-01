use std::fs::File;
use std::io::Write;
use std::io::Read;
use anyhow::Result;
use fuel_core::database::Database;
use fuel_core::database::Column;
use fuel_core::state::DataSource;
use fuel_core::state::in_memory::memory_store::MemoryStore;
use fuel_core::state::Value;
use fuel_core::types::blockchain::block::Block;
use fuels::tx::Bytes32;
use bincode;

use std::{ collections::BTreeMap, fmt::Debug, sync::{ Arc, Mutex } };

use crate::serialization::lib::BinFileSerde;

impl BinFileSerde for Block<Bytes32> {
  fn to_bincode_file(&self, path: String) -> Result<()> {
    let bin_data = bincode::serialize(&self)?;
    let mut file = File::create(&path)?;
    file.write_all(&bin_data)?;

    Ok(())
  }

  fn from_bincode_file(path: String) -> Result<Self> where Self: Sized {
    let mut file = File::open(&path)?;
    let mut bin_data = Vec::new();
    file.read_to_end(&mut bin_data)?;

    let block: Block<Bytes32> = bincode::deserialize(&bin_data)?;

    Ok(block)
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MyMemoryStore {
  inner: [Mutex<BTreeMap<Vec<u8>, Value>>; Column::COUNT],
}

// #[derive(serde::Serialize, serde::Deserialize)]
// struct DatabaseWrapper {
//   database: Database,
// }

// impl BinFileSerde for DatabaseWrapper {
//   fn to_bincode_file(&self, path: String) -> Result<()> {
//     let bin_data = bincode::serialize(&self)?;
//     let mut file = File::create(&path)?;
//     file.write_all(&bin_data)?;

//     Ok(())
//   }

//   fn from_bincode_file(path: String) -> Result<Self> where Self: Sized {
//     let mut file = File::open(&path)?;
//     let mut bin_data = Vec::new();
//     file.read_to_end(&mut bin_data)?;

//     let database: DatabaseWrapper = bincode::deserialize(&bin_data)?;

//     Ok(database)
//   }
// }
