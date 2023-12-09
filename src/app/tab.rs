mod days_frame;

use std::{collections::HashMap, ops::Not};

use anyhow::{Context, Result};
use chrono::{
  DateTime, Datelike, Days, Duration, Local, Month, NaiveDate,
  Timelike,
};
use tokio::sync::mpsc;

use self::days_frame::DaysFrame;

use super::{level::Level, selectable_list::SelectableList, Journal};

pub type Hour = u8;

#[derive(Default, Clone, PartialEq)]
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
      list: SelectableList::new().with_reversed_selection(true),
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
  days_frame: DaysFrame,
  app_changed_tx: mpsc::Sender<()>,
}

impl Tab {
  pub fn new(
    title: impl ToString,
    journal: Box<dyn Journal>,
    app_changed_tx: mpsc::Sender<()>,
  ) -> Self {
    let today = Local::now().date_naive();
    Self {
      journal,
      title: title.to_string(),
      state: State::new(today),
      undoes: vec![],
      redoes: vec![],
      days_frame: DaysFrame::new(today, Days::new(10)),
      app_changed_tx,
    }
  }

  pub fn resolve_all(&mut self) -> Result<()> {
    self.resolve_dates()?;
    self.state.recs_by_month = self.recs_by_month()?;
    Ok(())
  }

  fn resolve_dates(&mut self) -> Result<()> {
    self.state.date = self.days_frame.cur;
    *self.state.list = self.journal.day_records(self.state.date)?;
    self.state.level = self.level()?;
    self.state.recs_by_hour = self.recs_by_hour()?;
    self.state.recs_by_date =
      self.recs_for(self.days_frame.start, self.days_frame.end)?;
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
        let month = Month::try_from(date.month() as u8)
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
      .take_while(|date| date <= &end)
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

  pub fn prev_date(&mut self) -> Result<(), anyhow::Error> {
    self.days_frame.prev();
    self.resolve_dates()?;
    self.app_changed_tx.try_send(())?;
    Ok(())
  }

  pub fn next_date(&mut self) -> Result<(), anyhow::Error> {
    let s = self.days_frame.cur;
    self.days_frame.next();
    if s != self.days_frame.cur {
      self.resolve_dates()?;
      self.app_changed_tx.try_send(())?;
    }
    Ok(())
  }

  pub fn prev_selection(&mut self) -> Result<(), anyhow::Error> {
    let s = self.state.list.selected();
    self.state.list.select_prev();
    if s != self.state.list.selected() {
      self.app_changed_tx.try_send(())?;
    }
    Ok(())
  }

  pub fn next_selection(&mut self) -> Result<(), anyhow::Error> {
    let s = self.state.list.selected();
    self.state.list.select_next();
    if s != self.state.list.selected() {
      self.app_changed_tx.try_send(())?;
    }
    Ok(())
  }

  pub fn add_record(&mut self) -> Result<()> {
    let action = Action::Add(Local::now());
    self.apply(&action)?;
    self.undoes.push(!action);
    self.redoes.clear();
    self.resolve_dates()?;
    self.app_changed_tx.try_send(())?;
    Ok(())
  }

  fn apply(&mut self, action: &Action) -> Result<()> {
    match *action {
      Action::Add(dt) => {
        self.journal.add(dt)?;
        self.increment_month_counter(dt, 1);
      }
      Action::Delete(dt) => {
        self.journal.remove(dt)?;
        self.increment_month_counter(dt, -1);
      }
    }
    Ok(())
  }

  fn increment_month_counter(
    &mut self,
    dt: DateTime<Local>,
    dv: isize,
  ) {
    let month_counter = self.month_counter(dt);
    *month_counter = month_counter.saturating_add_signed(dv);
  }

  fn month_counter(&mut self, dt: DateTime<Local>) -> &mut usize {
    self
      .state
      .recs_by_month
      .entry(Month::try_from(dt.month() as u8).unwrap())
      .or_default()
  }

  pub fn delete_selected_record(&mut self) -> Result<()> {
    if let Some(&dt) = self.state.list.selected_item() {
      let action = Action::Delete(dt);
      self.apply(&action)?;
      self.undoes.push(!action);
      self.redoes.clear();
      self.resolve_dates()?;
      self.app_changed_tx.try_send(())?;
    }
    Ok(())
  }

  pub fn undo(&mut self) -> Result<()> {
    if let Some(action) = self.undoes.pop() {
      self.apply(&action)?;
      self.redoes.push(!action);
      self.resolve_dates()?;
      self.app_changed_tx.try_send(())?;
    }
    Ok(())
  }

  pub fn redo(&mut self) -> Result<()> {
    if let Some(action) = self.redoes.pop() {
      self.apply(&action)?;
      self.undoes.push(!action);
      self.resolve_dates()?;
      self.app_changed_tx.try_send(())?;
    }
    Ok(())
  }
}
