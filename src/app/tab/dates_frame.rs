use chrono::{Days, Duration, Local, NaiveDate};

pub struct DatesFrame {
  pub cur: NaiveDate,
  pub start: NaiveDate,
  pub end: NaiveDate,
}

impl DatesFrame {
  pub fn prev(&mut self) {
    let day = Duration::days(1);
    self.cur = self.cur.pred_opt().unwrap();
    if self.cur - self.middle() < -day {
      self.start -= day;
      self.end -= day;
    }
  }

  pub fn next(&mut self) {
    let day = Duration::days(1);
    let today = Local::now().date_naive();
    self.cur = self.cur.succ_opt().unwrap().min(today);
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
