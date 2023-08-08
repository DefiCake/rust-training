use anyhow::Error;
use log::{ info, warn, error };

// Use environment variable RUST_LOG=<level> to display different levels of logging
// e.g. RUST_LOG=info
fn main() {
  env_logger::init();

  info!("This is an info message");
  warn!("This is a warn message");
  error!("This is an error message");
}

pub fn sort(arr: &Vec<u64>) -> anyhow::Result<Vec<u64>, Error> {
  Ok(arr.clone())
}

#[cfg(test)] // Next line will only be used when testing
use rand::Rng;

#[test]
fn check_returns_arr() {
  let mut rng = rand::thread_rng();

  let len = rng.gen_range(1..100);

  let arr: Vec<u64> = (1..len).map(|_| rng.gen()).collect();
  let sorted_arr = sort(&arr).unwrap();

  assert_eq!(arr.len(), sorted_arr.len())
}
