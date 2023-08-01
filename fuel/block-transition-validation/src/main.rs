mod wallet;
mod serialization;
mod cli;
mod bootstrap;
mod load;

use cli::cli::{ Mode, mode };
use bootstrap::bootstrap;
use load::load;

#[tokio::main]
async fn main() {
  env_logger::init();

  match mode() {
    Mode::Bootstrap => {
      println!("BOOTSTRAP");
      bootstrap().await;
    }
    Mode::Load => {
      println!("LOAD");
      load().await;
    }
  }
}
