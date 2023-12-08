use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate};

pub type DayRecords = Vec<DateTime<Local>>;

pub trait Journal {
  fn day_records(&self, date: NaiveDate) -> Result<DayRecords>;
  fn add(&mut self, dt: DateTime<Local>) -> Result<()>;
  fn remove(&mut self, dt: DateTime<Local>) -> Result<()>;
}
