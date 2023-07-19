pub fn answer() -> i32 {
  42
}

// Tests are implemented in the same file where the code lives
#[test]
fn check_answer_validity() {
  assert_eq!(answer(), 42);
}
