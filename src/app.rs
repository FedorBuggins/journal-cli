use chrono::{Local, NaiveDate};

use crate::journal::{DayRecords, Journal};

pub struct App {
  journal: Journal,
  date: NaiveDate,
  should_quit: bool,
}

impl Default for App {
  fn default() -> Self {
    Self {
      journal: Journal,
      date: Local::now().date_naive(),
      should_quit: false,
    }
  }
}

impl App {
  pub fn date(&self) -> NaiveDate {
    self.date
  }

  pub fn prev_date(&mut self) {
    self.date = self.date.pred_opt().unwrap();
  }

  pub fn next_date(&mut self) {
    self.date =
      self.date.succ_opt().unwrap().min(Local::now().date_naive());
  }

  pub fn date_smoke_records(&self) -> DayRecords {
    self
      .journal
      .day_records(self.date)
      .expect("can't get today records")
  }

  pub fn smoke_records_for(
    &self,
    start: NaiveDate,
    end: NaiveDate,
  ) -> impl Iterator<Item = (NaiveDate, DayRecords)> + '_ {
    start
      .iter_days()
      .take_while(move |date| date <= &end)
      .flat_map(|date| {
        self.journal.day_records(date).map(|recs| (date, recs))
      })
  }

  pub fn add_smoke_record(&mut self) {
    self
      .journal
      .add(Local::now())
      .expect("can't add smoke record");
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }

  pub fn quit(&mut self) {
    self.should_quit = true;
  }
}
