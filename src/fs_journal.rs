use std::{
  io,
  path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDate};
use tokio::fs::{create_dir_all, read_to_string, try_exists, write};

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

#[async_trait]
impl Journal for FsJournal {
  async fn day_records(&self, date: NaiveDate) -> Result<DayRecords> {
    read_if_exist(&self.path(date))
      .await?
      .unwrap_or_default()
      .lines()
      .map(DateTime::parse_from_rfc3339)
      .map(|dt| dt.map(|dt| dt.with_timezone(&Local)))
      .collect::<Result<_, _>>()
      .context("date parse error")
  }

  async fn add(&self, dt: DateTime<Local>) -> Result<()> {
    let mut recs = self.day_records(dt.date_naive()).await?;
    recs.push(dt);
    recs.sort_unstable();
    create_dir_all(&self.dir).await?;
    write(
      self.path(dt.date_naive()),
      recs.into_iter().map(date_time_line).collect::<String>(),
    )
    .await?;
    Ok(())
  }

  async fn remove(&self, dt: DateTime<Local>) -> Result<()> {
    let recs = self.day_records(dt.date_naive()).await?;
    if !recs.is_empty() {
      write(
        self.path(dt.date_naive()),
        recs
          .into_iter()
          .filter(|rec| rec != &dt)
          .map(date_time_line)
          .collect::<String>(),
      )
      .await?;
    }
    Ok(())
  }
}

async fn read_if_exist(path: &Path) -> io::Result<Option<String>> {
  if try_exists(path).await? {
    read_to_string(path).await.map(Some)
  } else {
    Ok(None)
  }
}

fn date_time_line(dt: DateTime<Local>) -> String {
  dt.to_rfc3339() + "\n"
}
