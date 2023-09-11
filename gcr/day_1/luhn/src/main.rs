pub fn luhn(input: &str) -> bool {
  let cc_number = input.replace(" ", "");

  if cc_number.len() < 2 {
    return false;
  }

  let mut cc_number_reversed: Vec<char> = cc_number.chars().collect();
  cc_number_reversed.reverse();

  let mut sum: u32 = 0;
  for i in 0..cc_number_reversed.len() {
    if !cc_number_reversed[i].is_digit(10) {
      return false;
    }

    let n = cc_number_reversed[i].to_digit(10).unwrap();

    if i % 2 == 0 {
      sum += n;
    } else {
      let d: u32 = n * 2;
      if d < 10 {
        sum += d;
      } else {
        sum += d
          .to_string()
          .chars()
          .map(|c| c.to_digit(10).unwrap())
          .reduce(|acc, e| acc + e)
          .unwrap();
      }
    }
  }

  sum.to_string().ends_with("0")
}

#[test]
fn test_non_digit_cc_number() {
  assert!(!luhn("foo"));
  assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
  assert!(!luhn(""));
  assert!(!luhn(" "));
  assert!(!luhn("  "));
  assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
  assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
  assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
  assert!(luhn("4263 9826 4026 9299"));
  assert!(luhn("4539 3195 0343 6467"));
  assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
  assert!(!luhn("4223 9826 4026 9299"));
  assert!(!luhn("4539 3195 0343 6476"));
  assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
