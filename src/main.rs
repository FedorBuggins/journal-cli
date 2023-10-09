use std::{
  env,
  fs::{read_to_string, OpenOptions},
  io::{self, Write},
};

use chrono::{DateTime, Local, ParseError, Utc};

use args_parse::{parse, Command};

mod args_parse;

const HELP: &str = include_str!("../help.txt");
const DB: &str = "journal.txt";

fn main() {
  let cmd = parse(&env::args().skip(1).collect::<Vec<_>>())
    .unwrap_or(Command::Report);

  match cmd {
    Command::Add => add().unwrap(),
    Command::Report => report().unwrap(),
    Command::Help => println!("{HELP}"),
  }
}

fn add() -> io::Result<()> {
  let mut journal =
    OpenOptions::new().create(true).append(true).open(DB)?;
  journal.write((Utc::now().to_rfc3339() + "\n").as_bytes())?;
  println!("Record added");
  report()?;
  Ok(())
}

fn report() -> io::Result<()> {
  let mut counter = 0;
  for line in read_to_string(DB)?.lines().rev() {
    let dt =
      DateTime::parse_from_rfc3339(line).map_err(to_io_error)?;
    if dt.date_naive() < Utc::now().date_naive() {
      break;
    }
    println!("{}", dt.with_timezone(&Local).to_rfc2822());
    counter += 1;
  }
  println!("You smoked today {counter} cigarettes");
  Ok(())
}

fn to_io_error(err: ParseError) -> io::Error {
  io::Error::new(io::ErrorKind::Other, err)
}
