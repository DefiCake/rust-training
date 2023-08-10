use anyhow::Error;

// Use environment variable RUST_LOG=<level> to display different levels of logging
// e.g. RUST_LOG=info
fn main() {
  let arr = [0, 1, 2, 3, 4, 5, 6];
  let k = &arr.len() / 2; // 7 / 2 = 3
  dbg!(&arr[0..k]); // New thing learnt here: 0..3 for arrays does not include index 3
  dbg!(&arr[k..]);
}

pub fn sort(arr: &Vec<u64>) -> anyhow::Result<Vec<u64>, Error> {
  if arr.len() < 2 {
    return Ok(arr.clone());
  }

  let k = arr.len() / 2;

  let mut result: Vec<u64> = arr.clone();
  let mut sub_array_1 = sort(&result.drain(0..k).collect())?;
  let mut sub_array_2 = sort(&result.drain(0..).collect())?;

  while sub_array_1.len() > 0 {
    if sub_array_2.len() == 0 {
      return Ok([result, sub_array_1].concat());
    }

    if sub_array_1[0] < sub_array_2[0] {
      result.push(sub_array_1[0].clone());
      sub_array_1.rotate_left(1);
      sub_array_1.pop();
    } else {
      result.push(sub_array_2[0]);
      sub_array_2.rotate_left(1);
      sub_array_2.pop();
    }
  }

  Ok([result, sub_array_2].concat())
}

#[cfg(test)] // Next line will only be used when testing
mod tests {
  use crate::sort;
  use rand::Rng;

  #[test]
  fn test_returns_arr() {
    let mut rng = rand::thread_rng();

    let len = rng.gen_range(3..10);

    let arr: Vec<u64> = (1..len).map(|_| rng.gen()).collect();

    let sorted_arr = sort(&arr).unwrap();

    assert_eq!(arr.len(), sorted_arr.len())
  }

  #[test]
  fn test_sorts_ordered_array() {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(1..100);

    let unsorted_arr: Vec<u64> = (1..len).map(|_| rng.gen()).collect();
    let mut sorted_arr: Vec<u64> = unsorted_arr.clone();
    sorted_arr.sort();

    let result = sort(&unsorted_arr.into()).unwrap();

    for (pos, el) in sorted_arr.iter().enumerate() {
      let result_el: &u64 = &result[pos];
      assert_eq!(el, result_el, "Failed sort test");
    }
  }

  #[test]
  fn test_returns_arr_if_1_element() {
    let mut rng = rand::thread_rng();
    let len = 1;

    let unsorted_arr: Vec<u64> = (1..len).map(|_| rng.gen()).collect();
    let mut sorted_arr: Vec<u64> = unsorted_arr.clone();
    sorted_arr.sort();

    let result = sort(&unsorted_arr.into()).unwrap();

    for (pos, el) in sorted_arr.iter().enumerate() {
      let result_el: &u64 = &result[pos];
      assert_eq!(el, result_el, "Failed sort test");
    }
  }

  #[test]
  fn test_returns_arr_if_0_elements() {
    let empty_arr: Vec<u64> = [].into();
    let result = sort(&empty_arr).unwrap();

    assert_eq!(empty_arr, result);
  }
}
