use std::{env, io};

use chrono::{Duration, Local, Utc};

use crate::args_parse::{parse, Command};
use crate::journal::Journal;

mod args_parse;
mod journal;

const HELP: &str = include_str!("../help.txt");

fn main() {
  let cmd = parse(&env::args().skip(1).collect::<Vec<_>>())
    .unwrap_or(Command::Report(7));

  match cmd {
    Command::Add => add().unwrap(),
    Command::Details(n) => details(n).unwrap(),
    Command::Report(n) => report(n).unwrap(),
    Command::Help => println!("{HELP}"),
  }
}

fn add() -> io::Result<()> {
  Journal.add(Utc::now())?;
  println!("Record added");
  details(0)?;
  Ok(())
}

fn details(days_offset: usize) -> io::Result<()> {
  let date =
    (Utc::now() - Duration::days(days_offset as _)).date_naive();
  let records = Journal.get(date)?;
  for dt in &records {
    println!("{}", dt.with_timezone(&Local).to_rfc2822());
  }
  println!("You smoked {} cigarettes", records.len());
  Ok(())
}

fn report(days_offset: usize) -> io::Result<()> {
  let start =
    Utc::now().date_naive() - Duration::days(days_offset as i64 - 1);
  for date in start.iter_days().take(days_offset) {
    let records = Journal.get(date)?;
    println!("{} - {} cigarettes", date, records.len());
  }
  Ok(())
}
