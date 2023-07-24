use std::{ io::{ BufRead, BufReader, Read }, path::PathBuf };

use anyhow::{ Context, Result };
use clap::Parser;
use openssl::pkcs12::Pkcs12;

#[derive(Parser, Debug)]
struct Cli {
  path: Option<PathBuf>,
  pass: Option<String>,
}

fn main() -> Result<()> {
  let args = Cli::parse();

  let path = &args.path.unwrap_or_else(|| { PathBuf::from("test_artifacts/test_p12.p12") });
  let pass: &String = &args.pass.unwrap_or_else(|| { String::from("nopass") });

  let file = std::fs::File
    ::open(&path)
    .with_context(|| format!("Could not read path {}", path.clone().into_os_string().into_string().unwrap()))?;

  let mut buf_reader = BufReader::new(file);
  let mut buf = Vec::new();
  buf_reader.read_to_end(&mut buf)?;

  let pkcs12 = Pkcs12::from_der(&buf)?;

  // This gives Error: error:0308010C:digital envelope routines:inner_evp_generic_fetch:unsupported:crypto/evp/evp_fetch.c... it might have to do with OS dependencies,
  // need to enable legacy openssl
  let parsed = pkcs12.parse(&pass)?;

  // for (i, line) in std::io::BufReader::new(file).lines().enumerate() {
  //   match line {
  //     Ok(line_content) => {
  //       if line_content.contains(&args.pattern) {
  //         println!("{}", line_content);
  //       }
  //     }
  //     Err(err) => {
  //       eprintln!("Error reading line: {}: {}", i, err);
  //     }
  //   }
  // }

  Ok(())
}
