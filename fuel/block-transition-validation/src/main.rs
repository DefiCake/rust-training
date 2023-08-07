mod wallet;
mod serialization;
mod cli;
mod bootstrap;
mod load;
mod memstore;

use cli::cli::{ Mode, DBType, get_args };
use bootstrap::bootstrap;
use load::load;

#[tokio::main]
async fn main() {
  env_logger::init();

  let args = get_args();

  let mode = args.get_one::<Mode>("mode").expect("Required mode").clone();

  match mode {
    Mode::Bootstrap => {
      println!("BOOTSTRAP");
      let db_type = args.get_one::<DBType>("dbtype").expect("Required dbtype").clone();
      bootstrap(db_type).await;
    }
    Mode::Load => {
      println!("LOAD");
      load().await;
    }
  }
}
