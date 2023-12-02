use std::{
  fs::{read_to_string, write, OpenOptions},
  io::{self, Write},
  path::{Path, PathBuf},
};

use chrono::{DateTime, FixedOffset, Local, NaiveDate, ParseError};

pub type DayRecords = Vec<DateTime<FixedOffset>>;

pub struct Journal {
  dir: PathBuf,
}

impl Journal {
  pub fn new(dir: impl Into<PathBuf>) -> Self {
    Self { dir: dir.into() }
  }

  pub fn day_records(
    &self,
    date: NaiveDate,
  ) -> io::Result<DayRecords> {
    read_if_exist(&self.get_path(date))?
      .unwrap_or_default()
      .lines()
      .map(DateTime::parse_from_rfc3339)
      .collect::<Result<_, _>>()
      .map_err(to_io_error)
  }

  fn get_path(&self, date: NaiveDate) -> PathBuf {
    get_path(&self.dir, date)
  }

  pub fn add(&self, dt: DateTime<Local>) -> io::Result<()> {
    let date = dt.date_naive();
    let mut day_journal = OpenOptions::new()
      .create(true)
      .append(true)
      .open(self.get_path(date))?;
    day_journal.write_all((dt.to_rfc3339() + "\n").as_bytes())?;
    Ok(())
  }

  pub fn remove(&self, dt: DateTime<Local>) -> io::Result<()> {
    let recs = self.day_records(dt.date_naive())?;
    if !recs.is_empty() {
      write(
        self.get_path(dt.date_naive()),
        recs
          .into_iter()
          .filter(|rec| rec != &dt)
          .map(|dt| dt.to_rfc3339() + "\n")
          .collect::<String>(),
      )?;
    }
    Ok(())
  }
}

fn get_path(dir: &Path, date: NaiveDate) -> PathBuf {
  dir.join(format!("{date}.txt"))
}

fn read_if_exist(path: &Path) -> io::Result<Option<String>> {
  if Path::new(path).exists() {
    read_to_string(path).map(Some)
  } else {
    Ok(None)
  }
}

fn to_io_error(err: ParseError) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidData, err)
}
