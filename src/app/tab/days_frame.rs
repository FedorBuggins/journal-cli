use std::cmp::min;

use chrono::{Days, Duration, Local, NaiveDate};

#[derive(Clone, PartialEq)]
pub struct DaysFrame {
  pub cur: NaiveDate,
  pub start: NaiveDate,
  pub end: NaiveDate,
}

impl DaysFrame {
  pub fn new(end: NaiveDate, range: Days) -> Self {
    let cur = end;
    let start = end - range + Days::new(1);
    assert!(start < end, "invalid date range");
    Self { cur, start, end }
  }

  pub fn prev(&mut self) {
    let step = Duration::days(1);
    self.cur = self.cur.pred_opt().unwrap();
    if self.cur - self.middle() < -step {
      self.start -= step;
      self.end -= step;
    }
  }

  pub fn next(&mut self) {
    let today = Local::now().date_naive();
    let day = Duration::days(1);
    self.cur = min(self.cur.succ_opt().unwrap(), today);
    if self.cur - self.middle() > day && self.end < today {
      self.start += day;
      self.end += day;
    }
  }

  fn middle(&self) -> NaiveDate {
    let days_range =
      self.end.signed_duration_since(self.start).num_days() as u64;
    self.end - Days::new(days_range / 2)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_not_next_after_today() {
    let today = Local::now().date_naive();
    let mut df = DaysFrame::new(today, Days::new(5));
    assert_eq!(today, df.cur);
    df.next();
    assert_eq!(today, df.cur);
  }
}
