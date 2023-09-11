pub fn luhn(input: &str) -> bool {
  let cc_number = input.replace(" ", "");

  if cc_number.len() < 2 {
    return false;
  }

  let mut sum: u32 = 0;
  for (i, ch) in cc_number.chars().rev().enumerate() {
    if !ch.is_digit(10) {
      return false;
    }

    let n = ch.to_digit(10).unwrap();

    if i % 2 == 0 {
      sum += n;
      continue;
    }

    let d: u32 = n * 2;
    if d < 10 {
      sum += d;
      continue;
    }

    sum += d / 10 + (d % 10);
  }

  sum % 10 == 0
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
