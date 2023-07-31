use std::fs::File;
use clap::Parser;

#[derive(Parser)]
struct Args {
  path: String,
}

fn main() {
  let args: Args = Args::parse();
  let file = File::open(args.path);

  match file {
    Ok(_) => {
      println!("Could open file");
      std::process::exit(exitcode::OK);
    }
    Err(_) => {
      println!("Could not open path");
      std::process::exit(exitcode::IOERR);
    }
  }
}
