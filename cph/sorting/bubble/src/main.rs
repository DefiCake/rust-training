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
  let mut result = arr.clone();

  if result.len() < 1 {
    return Ok(result);
  }

  let mut i = 0;

  while i < result.len() {
    let mut j = 0;
    while j < result.len() - 1 {
      if result[j] > result[j + 1] {
        let aux = result[j];
        result[j] = result[j + 1];
        result[j + 1] = aux;
      }
      j += 1;
    }
    i += 1;
  }
  dbg!(&result);
  Ok(result)
}

#[cfg(test)] // Next line will only be used when testing
mod tests {
  use crate::sort;
  use rand::Rng;

  #[test]
  fn test_returns_arr() {
    let mut rng = rand::thread_rng();

    let len = rng.gen_range(1..100);

    let arr: Vec<u64> = (1..len).map(|_| rng.gen()).collect();
    let sorted_arr = sort(&arr).unwrap();

    assert_eq!(arr.len(), sorted_arr.len())
  }

  #[test]
  fn test_sorts_ordered_array() {
    let unsorted_arr: [u64; 15] = [1, 5, 4, 3, 6, 7, 8, 3, 1, 4, 5, 6, 7, 3, 2];
    let mut sorted_arr: [u64; 15] = unsorted_arr.clone();
    sorted_arr.sort();

    let result = sort(&unsorted_arr.into()).unwrap();

    for (pos, el) in sorted_arr.iter().enumerate() {
      let result_el: &u64 = &result[pos];
      assert_eq!(el, result_el, "Failed sort test");
    }
  }
}
