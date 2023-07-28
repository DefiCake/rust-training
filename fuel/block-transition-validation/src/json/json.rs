use std::{ borrow::Cow, fs::{ write, OpenOptions } };
use anyhow::Result;
use fuel_core::types::blockchain::block::Block;
use fuels::tx::Bytes32;

pub fn to_file(block: Cow<Block<Bytes32>>, path: String) -> Result<()> {
  let str = serde_json::to_string(&block)?;

  OpenOptions::new().create(true).write(true).truncate(true).open(&path)?;
  write(&path, str).expect("Failed to write to path");

  Ok(())
}
