use std::io;

use chrono::{DateTime, Local, NaiveDate};

pub type DayRecords = Vec<DateTime<Local>>;

pub trait Journal {
  fn day_records(&self, date: NaiveDate) -> io::Result<DayRecords>;
  fn add(&self, dt: DateTime<Local>) -> io::Result<()>;
  fn remove(&self, dt: DateTime<Local>) -> io::Result<()>;
}
