use std::fs::File;
use std::io::Write;
use std::io::Read;
use anyhow::Result;
use fuel_core::types::blockchain::block::Block;
use fuel_tx::Script;
use fuels::tx::Bytes32;
use bincode;

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

impl BinFileSerde for Script {
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

    let tx: Self = bincode::deserialize(&bin_data)?;

    Ok(tx)
  }
}
