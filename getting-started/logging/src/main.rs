use log::{ info, warn, error };
// Use environment variable RUST_LOG=<level> to display different levels of logging
// e.g. RUST_LOG=info
fn main() {
  env_logger::init();

  info!("This is an info message");
  warn!("This is a warn message");
  error!("This is an error message");
}
