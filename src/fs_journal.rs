use std::{
  fs::{create_dir_all, read_to_string, write},
  io,
  path::{Path, PathBuf},
};

use chrono::{DateTime, Local, NaiveDate, ParseError};

use crate::app::journal::{DayRecords, Journal};

pub struct FsJournal {
  dir: PathBuf,
}

impl FsJournal {
  pub fn new(dir: impl Into<PathBuf>) -> Self {
    Self { dir: dir.into() }
  }

  fn path(&self, date: NaiveDate) -> PathBuf {
    path(&self.dir, date)
  }
}

impl Journal for FsJournal {
  fn day_records(&self, date: NaiveDate) -> io::Result<DayRecords> {
    read_if_exist(&self.path(date))?
      .unwrap_or_default()
      .lines()
      .map(DateTime::parse_from_rfc3339)
      .map(|dt| dt.map(|dt| dt.with_timezone(&Local)))
      .collect::<Result<_, _>>()
      .map_err(to_io_error)
  }

  fn add(&self, dt: DateTime<Local>) -> io::Result<()> {
    let mut recs = self.day_records(dt.date_naive())?;
    recs.push(dt);
    recs.sort_unstable();
    create_dir_all(&self.dir)?;
    write(
      self.path(dt.date_naive()),
      recs
        .into_iter()
        .map(|dt| dt.to_rfc3339() + "\n")
        .collect::<String>(),
    )?;
    Ok(())
  }

  fn remove(&self, dt: DateTime<Local>) -> io::Result<()> {
    let recs = self.day_records(dt.date_naive())?;
    if !recs.is_empty() {
      write(
        self.path(dt.date_naive()),
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

fn path(dir: &Path, date: NaiveDate) -> PathBuf {
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
