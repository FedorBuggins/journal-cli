mod tab;

use std::{cmp::min, collections::HashMap, io};

use anyhow::Result;
use chrono::{DateTime, FixedOffset, Local, Month, NaiveDate};

use self::tab::Tab;

pub type DayRecords = Vec<DateTime<FixedOffset>>;

pub trait Journal {
  fn day_records(&self, date: NaiveDate) -> io::Result<DayRecords>;
  fn add(&self, dt: DateTime<Local>) -> io::Result<()>;
  fn remove(&self, dt: DateTime<Local>) -> io::Result<()>;
}

pub struct App {
  tabs: Vec<Tab>,
  selected_tab: usize,
  should_quit: bool,
}

pub struct State {
  pub tabs: SelectableList<String>,
  pub date: NaiveDate,
  pub list: SelectableList<DateTime<Local>>,
  pub recs_by_hour: HashMap<tab::Hour, usize>,
  pub recs_by_date: Vec<(NaiveDate, usize)>,
  pub recs_by_month: HashMap<Month, usize>,
}

#[derive(Default, Clone)]
pub struct SelectableList<T>
where
  T: Default + Clone,
{
  items: Vec<T>,
  selected: usize,
}

impl<T> SelectableList<T>
where
  T: Default + Clone,
{
  pub fn selected(&self) -> usize {
    min(self.selected, self.len().saturating_sub(1))
  }

  pub fn selected_item(&self) -> Option<&T> {
    self.get(self.selected())
  }

  fn with_selected(mut self, selected: usize) -> Self {
    self.select(selected);
    self
  }

  fn select(&mut self, selected: usize) {
    self.selected = min(selected, self.len().saturating_sub(1));
  }

  fn select_prev(&mut self) {
    self.select(self.selected().saturating_sub(1));
  }

  fn select_next(&mut self) {
    self.select(self.selected().saturating_add(1));
  }
}

impl<T> std::ops::Deref for SelectableList<T>
where
  T: Default + Clone,
{
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    &self.items
  }
}

impl<T> std::ops::DerefMut for SelectableList<T>
where
  T: Default + Clone,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.items
  }
}

impl<T> FromIterator<T> for SelectableList<T>
where
  T: Default + Clone,
{
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = T>,
  {
    Self {
      items: iter.into_iter().collect(),
      selected: 0,
    }
  }
}

impl App {
  pub fn try_new<S>(
    journals: Vec<(S, Box<dyn Journal>)>,
  ) -> Result<Self>
  where
    S: ToString,
  {
    let tabs =
      journals.into_iter().map(|(t, j)| Tab::new(t, j)).collect();
    let mut app = Self {
      tabs,
      selected_tab: 0,
      should_quit: false,
    };
    app.tab_mut().resolve_all()?;
    Ok(app)
  }

  fn tab_mut(&mut self) -> &mut Tab {
    &mut self.tabs[self.selected_tab]
  }

  pub fn state(&self) -> State {
    let tab_state = self.tab_state().clone();
    State {
      tabs: self
        .tabs
        .iter()
        .map(|tab| tab.title().clone())
        .collect::<SelectableList<_>>()
        .with_selected(self.selected_tab),
      date: tab_state.date,
      list: tab_state.list,
      recs_by_hour: tab_state.recs_by_hour,
      recs_by_date: tab_state.recs_by_date,
      recs_by_month: tab_state.recs_by_month,
    }
  }

  fn tab_state(&self) -> &tab::State {
    self.tabs[self.selected_tab].state()
  }

  pub fn next_tab(&mut self) -> Result<()> {
    self.selected_tab = (self.selected_tab + 1) % self.tabs.len();
    self.tab_mut().resolve_all()?;
    Ok(())
  }

  pub fn prev_date(&mut self) -> Result<()> {
    self.tab_mut().prev_date()
  }

  pub fn next_date(&mut self) -> Result<()> {
    self.tab_mut().next_date()
  }

  pub fn prev_record(&mut self) -> Result<()> {
    self.tab_mut().prev_record()
  }

  pub fn next_record(&mut self) -> Result<()> {
    self.tab_mut().next_record()
  }

  pub fn add_record(&mut self) -> Result<()> {
    self.tab_mut().add_record()
  }

  pub fn delete_selected_record(&mut self) -> Result<()> {
    self.tab_mut().delete_selected_record()
  }

  pub fn undo(&mut self) -> Result<()> {
    self.tab_mut().undo()
  }

  pub fn redo(&mut self) -> Result<()> {
    self.tab_mut().redo()
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }

  pub fn quit(&mut self) {
    self.should_quit = true;
  }
}
