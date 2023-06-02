use std::io::BufRead;

use clap::Parser;

#[derive(Parser)]
struct Cli {
  pattern: String,
  path: std::path::PathBuf,
}

fn main() {
  let args = Cli::parse();

  let file = match std::fs::File::open(&args.path) {
    Ok(file) => { file }
    Err(error) => { panic!("Could not read file, {}", error) }
  };

  for (i, line) in std::io::BufReader::new(file).lines().enumerate() {
    match line {
      Ok(line_content) => {
        if line_content.contains(&args.pattern) {
          println!("{}", line_content);
        }
      }
      Err(err) => {
        eprintln!("Error reading line: {}: {}", i, err);
      }
    }
  }
}
