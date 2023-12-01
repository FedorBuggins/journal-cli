use std::collections::HashMap;

use chrono::{
  DateTime, Datelike, Days, Duration, Local, Month, NaiveDate,
  Timelike,
};

use crate::journal::Journal;

pub type Hour = u8;

pub struct App {
  journal: Journal,
  state: State,
  undoes: Vec<DateTime<Local>>,
  redoes: Vec<DateTime<Local>>,
  should_quit: bool,
}

#[derive(Default)]
pub struct State {
  pub date: NaiveDate,
  pub date_smokes_by_hour: HashMap<Hour, usize>,
  pub recently_dates_smokes_count: Vec<(NaiveDate, usize)>,
  pub year_smokes_by_month: HashMap<Month, usize>,
}

impl App {
  pub fn new() -> Self {
    Self {
      journal: Journal,
      state: State::default(),
      undoes: Vec::new(),
      redoes: Vec::new(),
      should_quit: false,
    }
    .init()
  }

  fn init(mut self) -> Self {
    self.resolve(Local::now().date_naive());
    self
  }

  fn resolve(&mut self, date: NaiveDate) {
    let today = Local::now().date_naive();
    self.state.date = date;
    self.state.date_smokes_by_hour = self.date_smokes_by_hour();
    self.state.recently_dates_smokes_count = self
      .smoke_records_for(today - Days::new(9), today)
      .collect();
    self.state.year_smokes_by_month = self.year_smokes_by_month();
  }

  fn date_smokes_by_hour(&self) -> HashMap<Hour, usize> {
    self
      .journal
      .day_records(self.state.date)
      .unwrap_or_default()
      .into_iter()
      .fold(HashMap::new(), |mut map, dt| {
        *map.entry(dt.time().hour() as _).or_default() += 1;
        map
      })
  }

  fn year_smokes_by_month(&self) -> HashMap<Month, usize> {
    let today = Local::now().date_naive();

    self
      .smoke_records_for(today - Duration::days(365), today)
      .fold(HashMap::new(), |mut map, (date, recs_count)| {
        let month = Month::try_from(date.month0() as u8 + 1)
          .expect("invalid month number");
        *map.entry(month).or_default() += recs_count;
        map
      })
  }

  fn smoke_records_for(
    &self,
    start: NaiveDate,
    end: NaiveDate,
  ) -> impl Iterator<Item = (NaiveDate, usize)> + '_ {
    start.iter_days().take_while(move |date| date <= &end).map(
      |date| {
        (
          date,
          self.journal.day_records(date).unwrap_or_default().len(),
        )
      },
    )
  }

  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn prev_date(&mut self) {
    self.state.date =
      self.state.date.pred_opt().expect("can't get previous date");
    self.state.date_smokes_by_hour = self.date_smokes_by_hour();
  }

  pub fn next_date(&mut self) {
    self.state.date = self
      .state
      .date
      .succ_opt()
      .expect("can't get next date")
      .min(Local::now().date_naive());
    self.state.date_smokes_by_hour = self.date_smokes_by_hour();
  }

  pub fn add_smoke_record(&mut self) {
    let rec = Local::now();
    self.journal.add(rec).expect("can't add smoke record");
    self.undoes.push(rec);
    self.redoes = Vec::new();
    self.resolve(self.state.date);
  }

  pub fn undo(&mut self) {
    if let Some(rec) = self.undoes.pop() {
      self.journal.remove(rec).expect("can't remove smoke record");
      self.redoes.push(rec);
      self.resolve(self.state.date);
    }
  }

  pub fn redo(&mut self) {
    if let Some(rec) = self.redoes.pop() {
      self.journal.add(rec).expect("can't remove smoke record");
      self.undoes.push(rec);
      self.resolve(self.state.date);
    }
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }

  pub fn quit(&mut self) {
    self.should_quit = true;
  }
}
