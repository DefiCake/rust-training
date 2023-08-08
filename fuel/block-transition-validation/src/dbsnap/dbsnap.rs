use std::fs::{ OpenOptions, write };

use fuel_core::{ database::Database, chain_config::{ ChainConfig, StateConfig } };

pub fn snapshot(db: Database, path: String) -> anyhow::Result<()> {
  let chain_config: String = "local_testnet".to_string();

  let config: ChainConfig = chain_config.parse()?;
  let state_conf = StateConfig::generate_state_config(db)?;

  let chain_conf = ChainConfig {
    initial_state: Some(state_conf),
    ..config
  };

  let stringified = serde_json::to_string_pretty(&chain_conf)?;

  OpenOptions::new().create(true).write(true).truncate(true).open(&path)?;
  write(&path, stringified)?;

  Ok(())
}
