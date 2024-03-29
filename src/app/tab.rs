mod days_frame;

use std::{collections::HashMap, ops::Not};

use anyhow::{Context, Result};
use chrono::{
  DateTime, Datelike, Days, Duration, Local, Month, NaiveDate,
  Timelike,
};
use futures::future::try_join_all;
use tokio::sync::watch;

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

#[derive(Clone)]
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
  title: String,
  target: usize,
  journal: Box<dyn Journal>,
  state: State,
  undoes: Vec<Action>,
  redoes: Vec<Action>,
  days_frame: DaysFrame,
  state_tx: watch::Sender<State>,
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
impl Tab {
  pub fn new(
    title: impl Into<String>,
    target: usize,
    journal: Box<dyn Journal>,
  ) -> Self {
    let today = Local::now().date_naive();
    let state = State::new(today);
    let (state_tx, _) = watch::channel(state.clone());

    Self {
      title: title.into(),
      target,
      journal,
      state,
      undoes: vec![],
      redoes: vec![],
      days_frame: DaysFrame::new(today, Days::new(10)),
      state_tx,
    }
  }

  pub async fn resolve_all(&mut self) -> Result<()> {
    self.resolve().await?;
    self.state.recs_by_month = self.recs_by_month().await?;
    self.emit_changes()?;
    Ok(())
  }

  async fn resolve(&mut self) -> Result<()> {
    self.state.date = self.days_frame.cur;
    self.emit_changes()?;
    *self.state.list =
      self.journal.day_records(self.state.date).await?;
    self.emit_changes()?;
    self.state.level = self.level().await?;
    self.emit_changes()?;
    self.state.recs_by_hour = self.recs_by_hour().await?;
    self.emit_changes()?;
    self.state.recs_by_date = self
      .recs_for(self.days_frame.start, self.days_frame.end)
      .await?;
    self.emit_changes()?;
    Ok(())
  }

  async fn recs_by_hour(&self) -> Result<HashMap<Hour, usize>> {
    let recs = self
      .journal
      .day_records(self.state.date)
      .await?
      .into_iter()
      .fold(HashMap::new(), |mut map, dt| {
        *map.entry(dt.time().hour() as _).or_default() += 1;
        map
      });
    Ok(recs)
  }

  async fn recs_by_month(&self) -> Result<HashMap<Month, usize>> {
    let today = Local::now().date_naive();
    self
      .recs_for(today - Duration::days(365), today)
      .await?
      .into_iter()
      .try_fold(HashMap::new(), |mut map, (date, recs_count)| {
        let month = Month::try_from(date.month() as u8)
          .context("invalid month")?;
        *map.entry(month).or_default() += recs_count;
        Ok(map)
      })
  }

  async fn recs_for(
    &self,
    start: NaiveDate,
    end: NaiveDate,
  ) -> Result<Vec<(NaiveDate, usize)>> {
    let recs = start.iter_days().take_while(|date| date <= &end).map(
      |date| async move {
        Ok((date, self.journal.day_records(date).await?.len()))
      },
    );
    try_join_all(recs).await
  }

  async fn level(&self) -> Result<Level> {
    let today = self.state.date;
    let recent_days = self
      .recs_for(today - Days::new(7), today - Days::new(1))
      .await?;
    let recent_days_iter =
      recent_days.iter().map(|(_, recs)| *recs as f32);
    let recent_days_sum: f32 = recent_days_iter.clone().sum();
    let recent_days_count = recent_days_iter
      .filter(|count| (count - 0.).abs() > f32::EPSILON)
      .count();
    let date_count =
      self.journal.day_records(self.state.date).await?.len();
    let middle = if recent_days_count == 0 {
      date_count as f32
    } else {
      recent_days_sum / recent_days_count as f32
    };
    Ok(Level::new(date_count, middle, self.target))
  }

  fn emit_changes(&self) -> Result<()> {
    self.state_tx.send(self.state.clone())?;
    Ok(())
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn subscribe(&self) -> watch::Receiver<State> {
    self.state_tx.subscribe()
  }

  pub async fn prev_date(&mut self) -> Result<()> {
    self.days_frame.prev();
    self.resolve().await?;
    Ok(())
  }

  pub async fn next_date(&mut self) -> Result<()> {
    self.days_frame.next();
    self.resolve().await?;
    Ok(())
  }

  pub fn prev_selection(&mut self) -> Result<()> {
    let s = self.state.list.selected();
    self.state.list.select_prev();
    if s != self.state.list.selected() {
      self.emit_changes()?;
    }
    Ok(())
  }

  pub fn next_selection(&mut self) -> Result<()> {
    let s = self.state.list.selected();
    self.state.list.select_next();
    if s != self.state.list.selected() {
      self.emit_changes()?;
    }
    Ok(())
  }

  pub async fn add_record(&mut self) -> Result<()> {
    let action = Action::Add(Local::now());
    self.apply(&action).await?;
    self.undoes.push(!action);
    self.redoes.clear();
    self.resolve().await?;
    Ok(())
  }

  async fn apply(&mut self, action: &Action) -> Result<()> {
    match *action {
      Action::Add(dt) => {
        self.journal.add(dt).await?;
        self.increment_month_counter(dt, 1);
      }
      Action::Delete(dt) => {
        self.journal.remove(dt).await?;
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

  pub async fn delete_selected_record(&mut self) -> Result<()> {
    if let Some(&dt) = self.state.list.selected_item() {
      let action = Action::Delete(dt);
      self.apply(&action).await?;
      self.undoes.push(!action);
      self.redoes.clear();
      self.resolve().await?;
    }
    Ok(())
  }

  pub async fn undo(&mut self) -> Result<()> {
    if let Some(action) = self.undoes.pop() {
      self.apply(&action).await?;
      self.redoes.push(!action);
      self.resolve().await?;
    }
    Ok(())
  }

  pub async fn redo(&mut self) -> Result<()> {
    if let Some(action) = self.redoes.pop() {
      self.apply(&action).await?;
      self.undoes.push(!action);
      self.resolve().await?;
    }
    Ok(())
  }

  pub fn demount(&mut self) -> Result<()> {
    self.state = State::new(self.state.date);
    self.emit_changes()?;
    Ok(())
  }
}
