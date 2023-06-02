use std::io::BufRead;

use anyhow::{ Context, Result };
use clap::Parser;

#[derive(Parser)]
struct Cli {
  pattern: String,
  path: std::path::PathBuf,
}

fn main() -> Result<()> {
  let args = Cli::parse();

  let file = std::fs::File
    ::open(&args.path)
    .with_context(|| format!("Could not read path {}", &args.path.into_os_string().into_string().unwrap()))?;

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

  Ok(())
}
