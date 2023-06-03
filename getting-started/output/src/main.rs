// This line does two things:
//  - self makes io:: directly available without needing to do std::io
//  - Write makes the methods {writeln, write} directly available without needing to do std::io::Write
use std::io::{ self, Write };

fn main() {
  let stdout = io::stdout(); // get the global stdout entity

  // Wrapping stdout into a buffer means that the buffer acts as an intermediary
  // The buffe will only write its contents (and remove them) in the following conditions:
  //  - We issue a new line (\n) into its contents (this is done automatically if we use writeln!)
  //  - We call handle.flush()
  //  - The buffer 's capacity is exceeded. To avoid loss of data, the buffer flushes an amount of data equal to what would have been lost
  let mut handle = io::BufWriter::new(stdout);

  // The pro of this method is that it does less calls to print lines, which are computationally expensive
  // The con is that in the event of an error or unexpected program closure, data in the buffer are lost
  writeln!(handle, "foo: {}", 42).unwrap(); // add `?` if you care about errors here

  write!(handle, "buffered data {}", 4).unwrap();
  write!(handle, " more buffered data {}", 16).unwrap(); // Data is concatenated, so add a space after it

  handle.flush().unwrap();
}
