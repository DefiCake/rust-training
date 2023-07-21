use std::error::Error;
use serde::{ Serialize, Deserialize };

// Note that in order to use the derive for serde, it has to be activated in Cargo.toml features
#[derive(Serialize, Deserialize)]
struct Config {
  username: String,
  password: String,
}

impl std::default::Default for Config {
  fn default() -> Self {
    Self { username: "Write your username".into(), password: "Write your password".into() }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let cfg: Config = confy::load("app_name", "DEVELOPMENT")?;

  println!("Here is the config!\t\t {}:{}", cfg.username.clone(), cfg.password.clone());

  Ok(())
}
