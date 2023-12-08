use std::{
  fs::{create_dir_all, read_to_string, write},
  io,
  path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate};

use crate::app::journal::{DayRecords, Journal};

pub struct FsJournal {
  dir: PathBuf,
}

impl FsJournal {
  pub fn new(dir: impl Into<PathBuf>) -> Self {
    Self { dir: dir.into() }
  }

  fn path(&self, date: NaiveDate) -> PathBuf {
    self.dir.join(format!("{date}.txt"))
  }
}

impl Journal for FsJournal {
  fn day_records(&self, date: NaiveDate) -> Result<DayRecords> {
    read_if_exist(&self.path(date))?
      .unwrap_or_default()
      .lines()
      .map(DateTime::parse_from_rfc3339)
      .map(|dt| dt.map(|dt| dt.with_timezone(&Local)))
      .collect::<Result<_, _>>()
      .context("date parse error")
  }

  fn add(&mut self, dt: DateTime<Local>) -> Result<()> {
    let mut recs = self.day_records(dt.date_naive())?;
    recs.push(dt);
    recs.sort_unstable();
    create_dir_all(&self.dir)?;
    write(
      self.path(dt.date_naive()),
      recs.into_iter().map(date_time_line).collect::<String>(),
    )?;
    Ok(())
  }

  fn remove(&mut self, dt: DateTime<Local>) -> Result<()> {
    let recs = self.day_records(dt.date_naive())?;
    if !recs.is_empty() {
      write(
        self.path(dt.date_naive()),
        recs
          .into_iter()
          .filter(|rec| rec != &dt)
          .map(date_time_line)
          .collect::<String>(),
      )?;
    }
    Ok(())
  }
}

fn read_if_exist(path: &Path) -> io::Result<Option<String>> {
  if Path::new(path).exists() {
    read_to_string(path).map(Some)
  } else {
    Ok(None)
  }
}

fn date_time_line(dt: DateTime<Local>) -> String {
  dt.to_rfc3339() + "\n"
}
