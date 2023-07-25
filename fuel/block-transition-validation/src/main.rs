use fuel_core::{ database::Database, chain_config::ChainConfig, service::{ Config, FuelService } };

#[tokio::main]
async fn main() {
  env_logger::init();

  let database = Database::in_memory();
  let chain_config = ChainConfig::local_testnet();
  let node_config = Config::local_node();
  let service_config: Config = Config {
    chain_conf: chain_config,
    ..node_config
  };

  let _service = FuelService::from_database(database.clone(), service_config).await.unwrap();

  let genesis = database.get_genesis().unwrap();
  println!("Genesis data: {:?}", genesis);
}
