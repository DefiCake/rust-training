use std::{ borrow::Cow, fs::{ write, OpenOptions, File } };
use anyhow::Result;
use fuel_core::types::blockchain::block::Block;
use fuels::tx::Bytes32;

#[allow(dead_code)]
pub fn to_json_file(block: &Cow<Block<Bytes32>>, path: String) -> Result<()> {
  let str = serde_json::to_string(&block)?;
  OpenOptions::new().create(true).write(true).truncate(true).open(&path)?;
  write(&path, str).expect("Failed to write to path");

  Ok(())
}

#[allow(dead_code)]
pub fn from_json_file(path: String) -> Result<Block<Bytes32>> {
  let f = File::open(path)?;
  let block: Block<Bytes32> = serde_json::from_reader(f)?;

  Ok(block)
}
