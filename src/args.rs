use clap::{ Arg, Command, Parser, parser::ValueSource };

#[derive(Parser)]
struct Args {
  r: String,
  i: String,
  // only if directory is given
  d: String,
}

fn cli() -> Command {
  Command::new("rs")
    .args([Arg::new("i").long("i")
      .value_name("WHEN")
      .value_parser(["always", "auto", "never"])
      .default_value("auto")
      .num_args(0..2)
      .require_equals(true)
      .default_missing_value("always")
      .help("info"), 
    ])
}

pub fn asserting() {
  let m  = cli().get_matches_from(vec![
    "rs"
  ]);
  assert_eq!(m.get_one::<String>("i").unwrap(), "auto");
  assert_eq!(m.value_source("i"), Some(ValueSource::DefaultValue));

  let m  = cli().get_matches_from(vec![
    "rs", "--i=never"
  ]);
  assert_eq!(m.get_one::<String>("i").unwrap(), "never");
  assert_eq!(m.value_source("i"), Some(ValueSource::CommandLine));

  let m  = cli().get_matches_from(vec![
    "rs", "--i"
  ]);
  assert_eq!(m.get_one::<String>("i").unwrap(), "always");
  assert_eq!(m.value_source("i"), Some(ValueSource::CommandLine)); 
}


