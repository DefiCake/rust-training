// To include files in the rust package, use `mod` and the name of the file
// you want to import. Note that it must be a file at the same level of `main.rs`
mod others;

// Now in that file, there are declarations of other several different files
// Check others.rs to see how it is done, but this means:
// look for `answer()` inside `answer.rs` at `others.rs`
use others::*;

fn main() {
  let a = answer::answer();
  println!("Answer is {}", a);

  let b = another_answer::answer();
  println!("Another answer is {}", b);
}
