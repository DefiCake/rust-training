use std::fs::File;
use std::io::Write;
use std::io::Read;
use anyhow::Result;
use fuel_core::types::blockchain::block::Block;
use fuels::tx::Bytes32;
use std::borrow::Cow;
use bincode;

pub fn to_bincode_file(block: &Cow<Block<Bytes32>>, path: String) -> Result<()> {
  let bin_data = bincode::serialize(&block)?;
  let mut file = File::create(&path)?;
  file.write_all(&bin_data)?;

  Ok(())
}

pub fn from_bincode_file(path: String) -> Result<Block<Bytes32>> {
  let mut file = File::open(&path)?;
  let mut bin_data = Vec::new();
  file.read_to_end(&mut bin_data)?;

  let block: Block<Bytes32> = bincode::deserialize(&bin_data)?;

  Ok(block)
}
