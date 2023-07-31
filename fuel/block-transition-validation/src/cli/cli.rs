use clap::{ arg, Command, builder::EnumValueParser, ValueEnum };

#[derive(Clone, ValueEnum)]
pub enum Mode {
  Bootstrap,
  Load,
}

pub fn mode() -> Mode {
  let matches = Command::new("block-transition-validation")
    .version("1.0")
    .arg(arg!(-m --mode <VALUE>).required(true).value_parser(EnumValueParser::<Mode>::new()))
    .get_matches();

  matches.get_one::<Mode>("mode").expect("Required mode").clone()
}
