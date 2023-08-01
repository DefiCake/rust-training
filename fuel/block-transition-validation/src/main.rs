mod wallet;
mod serialization;
mod cli;
mod bootstrap;

use bootstrap::bootstrap;

use cli::cli::{ Mode, mode };

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
    }
  }
}
