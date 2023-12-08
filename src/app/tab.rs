mod dates_frame;

use std::{collections::HashMap, ops::Not};

use anyhow::{Context, Ok, Result};
use chrono::{
  DateTime, Datelike, Days, Duration, Local, Month, NaiveDate,
  Timelike,
};

use self::dates_frame::DatesFrame;

use super::{
  level::Level, selectable_list::SelectableList, Command, Journal,
};

pub type Hour = u8;

#[derive(Default, Clone)]
pub struct State {
  pub date: NaiveDate,
  pub list: SelectableList<DateTime<Local>>,
  pub level: Level,
  pub recs_by_hour: HashMap<Hour, usize>,
  pub recs_by_date: Vec<(NaiveDate, usize)>,
  pub recs_by_month: HashMap<Month, usize>,
}

impl State {
  fn new(date: NaiveDate) -> Self {
    Self {
      date,
      list: SelectableList::default().with_reversed_selection(),
      ..Default::default()
    }
  }
}

enum Action {
  Add(DateTime<Local>),
  Delete(DateTime<Local>),
}

impl Not for Action {
  type Output = Self;

  fn not(self) -> Self::Output {
    match self {
      Action::Add(dt) => Action::Delete(dt),
      Action::Delete(dt) => Action::Add(dt),
    }
  }
}

pub struct Tab {
  journal: Box<dyn Journal>,
  title: String,
  state: State,
  undoes: Vec<Action>,
  redoes: Vec<Action>,
  dates_frame: DatesFrame,
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
    *self.state.list = self.journal.day_records(self.state.date)?;
    self.state.level = self.level()?;
    self.state.recs_by_hour = self.recs_by_hour()?;
    self.state.recs_by_date =
      self.recs_for(self.dates_frame.start, self.dates_frame.end)?;
    Ok(())
  }

  fn recs_by_hour(&self) -> Result<HashMap<Hour, usize>> {
    Ok(self.journal.day_records(self.state.date)?.into_iter().fold(
      HashMap::new(),
      |mut map, dt| {
        *map.entry(dt.time().hour() as _).or_default() += 1;
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

  fn level(&self) -> Result<Level> {
    let today = Local::now().date_naive();
    let date_count =
      self.journal.day_records(self.state.date)?.len() as f32;
    let recent_days =
      self.recs_for(today - Days::new(10), today - Days::new(1))?;
    let sum: f32 =
      recent_days.iter().map(|(_, recs)| *recs as f32).sum();
    let count =
      recent_days.iter().filter(|(_, count)| count != &0).count();
    let middle = if count == 0 {
      date_count
    } else {
      sum / count as f32
    };
    Ok(Level::new(date_count / middle, middle))
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn handle_cmd(&mut self, cmd: Command) -> Result<()> {
    match cmd {
      Command::PrevDate => self.prev_date()?,
      Command::NextDate => self.next_date()?,
      Command::PrevSelection => self.state.list.select_prev(),
      Command::NextSelection => self.state.list.select_next(),
      Command::AddRecord => self.add_record()?,
      Command::DeleteSelectedRecord => {
        self.delete_selected_record()?
      }
      Command::Undo => self.undo()?,
      Command::Redo => self.redo()?,
      _ => (),
    }
    Ok(())
  }

  fn prev_date(&mut self) -> Result<()> {
    self.dates_frame.prev();
    self.resolve_dates()?;
    Ok(())
  }

  fn next_date(&mut self) -> Result<()> {
    self.dates_frame.next();
    self.resolve_dates()?;
    Ok(())
  }

  fn add_record(&mut self) -> Result<()> {
    let rec = Local::now();
    self.journal.add(rec)?;
    self.undoes.push(Action::Delete(rec));
    self.redoes.clear();
    self.resolve_all()?;
    Ok(())
  }

  fn delete_selected_record(&mut self) -> Result<()> {
    if let Some(selected_rec) = self.state.list.selected_item() {
      let dt = selected_rec.with_timezone(&Local);
      self.journal.remove(dt)?;
      self.undoes.push(Action::Add(dt));
      self.redoes.clear();
      self.resolve_all()?;
    }
    Ok(())
  }

  fn undo(&mut self) -> Result<()> {
    if let Some(action) = self.undoes.pop() {
      self.execute(&action)?;
      self.redoes.push(!action);
      self.resolve_all()?;
    }
    Ok(())
  }

  fn execute(&mut self, action: &Action) -> Result<()> {
    match *action {
      Action::Add(rec) => self.journal.add(rec)?,
      Action::Delete(rec) => self.journal.remove(rec)?,
    }
    Ok(())
  }

  fn redo(&mut self) -> Result<()> {
    if let Some(action) = self.redoes.pop() {
      self.execute(&action)?;
      self.undoes.push(!action);
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
