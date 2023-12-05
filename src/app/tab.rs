mod dates_frame;

use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::{
  DateTime, Datelike, Days, Duration, Local, Month, NaiveDate,
  Timelike,
};

use self::dates_frame::DatesFrame;

use super::Journal;

pub type Hour = u8;

pub struct Tab {
  journal: Box<dyn Journal>,
  title: String,
  state: State,
  undoes: Vec<DateTime<Local>>,
  redoes: Vec<DateTime<Local>>,
  dates_frame: DatesFrame,
}

#[derive(Default, Clone)]
pub struct State {
  pub date: NaiveDate,
  pub recs_by_hour: HashMap<Hour, usize>,
  pub recs_by_date: Vec<(NaiveDate, usize)>,
  pub recs_by_month: HashMap<Month, usize>,
}

impl State {
  fn new(date: NaiveDate) -> Self {
    Self {
      date,
      ..Default::default()
    }
  }
}

impl Tab {
  pub fn new(
    title: impl ToString,
    journal: Box<dyn Journal>,
  ) -> Self {
    let today = Local::now().date_naive();
    Self {
      journal,
      title: title.to_string(),
      state: State::new(today),
      undoes: vec![],
      redoes: vec![],
      dates_frame: DatesFrame {
        cur: today,
        start: today - Days::new(9),
        end: today,
      },
    }
  }

  pub fn resolve_all(&mut self) -> Result<()> {
    self.resolve_dates()?;
    self.state.recs_by_month = self.recs_by_month()?;
    Ok(())
  }

  pub fn resolve_dates(&mut self) -> Result<()> {
    self.state.date = self.dates_frame.cur;
    self.state.recs_by_hour = self.recs_by_hour()?;
    self.state.recs_by_date =
      self.recs_for(self.dates_frame.start, self.dates_frame.end)?;
    Ok(())
  }

  fn recs_by_hour(&self) -> Result<HashMap<Hour, usize>> {
    Ok(self.journal.day_records(self.state.date)?.into_iter().fold(
      HashMap::new(),
      |mut map, dt| {
        let mut time = dt.time();
        if time.minute() >= 50 && time.hour() < 23 {
          time += Duration::hours(1);
        }
        *map.entry(time.hour() as _).or_default() += 1;
        map
      },
    ))
  }

  fn recs_by_month(&self) -> Result<HashMap<Month, usize>> {
    let today = Local::now().date_naive();
    self
      .recs_for(today - Duration::days(365), today)?
      .into_iter()
      .try_fold(HashMap::new(), |mut map, (date, recs_count)| {
        let month = Month::try_from(date.month0() as u8 + 1)
          .context("invalid month")?;
        *map.entry(month).or_default() += recs_count;
        Ok(map)
      })
  }

  fn recs_for(
    &self,
    start: NaiveDate,
    end: NaiveDate,
  ) -> Result<Vec<(NaiveDate, usize)>> {
    start
      .iter_days()
      .take_while(move |date| date <= &end)
      .map(|date| Ok((date, self.journal.day_records(date)?.len())))
      .collect()
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn prev_date(&mut self) -> Result<()> {
    self.dates_frame.prev();
    self.resolve_dates()?;
    Ok(())
  }

  pub fn next_date(&mut self) -> Result<()> {
    self.dates_frame.next();
    self.resolve_dates()?;
    Ok(())
  }

  pub fn add_record(&mut self) -> Result<()> {
    let rec = Local::now();
    self.journal.add(rec)?;
    self.undoes.push(rec);
    self.redoes = vec![];
    self.resolve_all()?;
    Ok(())
  }

  pub fn undo(&mut self) -> Result<()> {
    if let Some(rec) = self.undoes.pop() {
      self.journal.remove(rec)?;
      self.redoes.push(rec);
      self.resolve_all()?;
    }
    Ok(())
  }

  pub fn redo(&mut self) -> Result<()> {
    if let Some(rec) = self.redoes.pop() {
      self.journal.add(rec)?;
      self.undoes.push(rec);
      self.resolve_all()?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod learning_tests {
  use chrono::{Duration, NaiveTime};

  #[test]
  fn time_add_dutation_with_overflow() {
    let time = NaiveTime::from_hms_opt(23, 0, 0).unwrap();
    let result = time + Duration::hours(1);
    let expected = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    assert_eq!(result, expected);
  }
}
