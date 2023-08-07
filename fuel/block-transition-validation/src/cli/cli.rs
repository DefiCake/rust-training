use clap::{ arg, Command, builder::EnumValueParser, ValueEnum, ArgMatches };

#[derive(Clone, ValueEnum)]
pub enum Mode {
  Bootstrap,
  Load,
}

#[derive(Clone, ValueEnum)]
pub enum DBType {
  Memory,
  Rocks,
}

pub fn get_args() -> ArgMatches {
  let matches = Command::new("block-transition-validation")
    .version("1.0")
    .arg(arg!(-m --mode <VALUE>).required(true).value_parser(EnumValueParser::<Mode>::new()))
    .arg(arg!(-d --dbtype <VALUE>).required_if_eq("mode", "bootstrap").value_parser(EnumValueParser::<DBType>::new()))
    .get_matches();

  matches
}
