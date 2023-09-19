// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn main() {
  let v: Vec<i8> = vec![10, 20, 30];
  let mut iter = v.iter();

  let v0: Option<&i8> = iter.next();
  println!("v0: {v0:?}");
}

fn two() {
  fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    let v0: Option<String> = iter.next();
    println!("v0: {v0:?}");
  }
}

fn loops() {
  fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
      println!("word: {word}");
    }

    for word in v {
      println!("word: {word}");
    }
  }
}

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
  let prefix_iter = prefix.split("/");
  let request_iter = request_path.split("/");

  if prefix_iter.clone().count() > request_iter.clone().count() {
    return false;
  }

  for (prefix_section, request_section) in prefix_iter.zip(request_iter) {
    if prefix_section == "*" {
      continue;
    }

    dbg!(prefix_section, request_section);
    if prefix_section != request_section {
      return false;
    }
  }

  true
}

#[test]
fn test_matches_without_wildcard() {
  assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
  assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
  assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

  assert!(!prefix_matches("/v1/publishers", "/v1"));
  assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
  assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
  assert!(prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/books"));
  assert!(prefix_matches("/v1/publishers/*/books", "/v1/publishers/bar/books"));
  assert!(prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/books/book1"));

  assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
  assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers/foo/booksByAuthor"));
}
