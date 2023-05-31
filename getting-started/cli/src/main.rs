use std::io::BufRead;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();

    let mut content = std::io::BufReader::new(std::fs::File::open(&args.path).expect("Could not read file"));
    
    let mut line = String::new();

    while content.read_line(&mut line).expect("Could not read contents inside file") > 0 {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
        line.clear();
    }
}
