use std::io::BufRead;

use clap::Parser;

#[derive(Parser)]
struct Cli {
  pattern: String,
  path: std::path::PathBuf,
}

fn main() {
  let args = Cli::parse();

  // let mut content = std::io::BufReader::new(std::fs::File::open(&args.path).expect("Could not read file"));

  let file = match std::fs::File::open(&args.path) {
    Ok(file) => { file }
    Err(error) => { panic!("Could not read file, {}", error) }
  };

  for line in std::io::BufReader::new(file).lines() {
    match line {
      Ok(line_content) => {
        if line_content.contains(&args.pattern) {
          println!("{}", line_content);
        }
      }
      Err(err) => {
        eprintln!("Error reading line: {}", err);
      }
    }
  }
}
