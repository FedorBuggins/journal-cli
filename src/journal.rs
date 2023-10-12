use std::{
  fs::{read_to_string, OpenOptions},
  io::{self, Write},
  path::Path,
};

use chrono::{DateTime, FixedOffset, NaiveDate, ParseError, Utc};

const DATA_FOLDER: &str = "journal";

pub type Records = Vec<DateTime<FixedOffset>>;

pub struct Journal;

impl Journal {
  pub fn get(&self, date: NaiveDate) -> io::Result<Records> {
    read_if_exist(&format!("{DATA_FOLDER}/{date}.txt"))?
      .unwrap_or_default()
      .lines()
      .map(DateTime::parse_from_rfc3339)
      .collect::<Result<_, _>>()
      .map_err(to_io_error)
  }

  pub fn add(&self, dt: DateTime<Utc>) -> io::Result<()> {
    let date = dt.date_naive();
    let mut journal = OpenOptions::new()
      .create(true)
      .append(true)
      .open(format!("{DATA_FOLDER}/{date}.txt"))?;
    journal.write((dt.to_rfc3339() + "\n").as_bytes())?;
    Ok(())
  }
}

fn read_if_exist(path: &str) -> io::Result<Option<String>> {
  if Path::new(path).exists() {
    read_to_string(path).map(Some)
  } else {
    Ok(None)
  }
}

fn to_io_error(err: ParseError) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidData, err)
}
