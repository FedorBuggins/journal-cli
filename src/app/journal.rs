use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDate};

pub type DayRecords = Vec<DateTime<Local>>;

#[async_trait]
pub trait Journal: Send + Sync {
  async fn day_records(&self, date: NaiveDate) -> Result<DayRecords>;
  async fn add(&self, dt: DateTime<Local>) -> Result<()>;
  async fn remove(&self, dt: DateTime<Local>) -> Result<()>;
}
