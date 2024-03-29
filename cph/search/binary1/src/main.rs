use log::{ info, warn, error };

// Use environment variable RUST_LOG=<level> to display different levels of logging
// e.g. RUST_LOG=info
fn main() {
  env_logger::init();

  info!("This is an info message");
  warn!("This is a warn message");
  error!("This is an error message");
}
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
  Empty,
  NotFound,
}

pub fn search(arr: &Vec<u64>, elem: &u64) -> anyhow::Result<usize, SearchError> {
  if arr.len() == 0 {
    return Err(SearchError::Empty);
  }

  let mut a: usize = 0;
  let mut b: usize = arr.len() - 1;

  while a <= b {
    let bsc = (a + b) / 2;

    if arr[bsc] == *elem {
      return Ok(bsc);
    }

    match arr[bsc] > *elem {
      true => {
        b = bsc - 1;
      }
      false => {
        a = bsc + 1;
      }
    }
  }

  Err(SearchError::NotFound)
}

#[cfg(test)] // Next line will only be used when testing
mod tests {
  use crate::{ search, SearchError };
  use rand::Rng;

  #[test]
  fn test_search_elem() {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(1..100);

    let mut arr: Vec<u64> = (1..len).map(|_| rng.gen()).collect();
    arr.sort();

    let idx: usize = rng.gen_range(1..len);
    let result = search(&arr, &arr[idx]).unwrap();

    assert_eq!(idx, result)
  }

  #[test]
  fn test_search_elem_empty() {
    let arr: Vec<u64> = [].to_vec();

    let result = search(&arr, &0);

    assert!(result.is_err());

    assert_eq!(result.err().unwrap(), SearchError::Empty);
  }

  #[test]
  fn test_search_elem_not_found() {
    let arr: Vec<u64> = [1, 2, 3, 4, 5, 6].to_vec();

    let result = search(&arr, &7);

    assert!(result.is_err());

    assert_eq!(result.err().unwrap(), SearchError::NotFound);
  }
}
