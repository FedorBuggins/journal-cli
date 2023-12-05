mod tab;

use std::collections::HashMap;

use anyhow::Result;
use chrono::{Month, NaiveDate};

use crate::journal::Journal;

use self::tab::Tab;

pub struct App {
  tabs: Vec<Tab>,
  selected_tab: usize,
  should_quit: bool,
}

pub struct State {
  pub tabs: Vec<String>,
  pub selected_tab: usize,
  pub date: NaiveDate,
  pub recs_by_hour: HashMap<tab::Hour, usize>,
  pub recs_by_date: Vec<(NaiveDate, usize)>,
  pub recs_by_month: HashMap<Month, usize>,
}

impl App {
  pub fn try_new<I, S>(journals: I) -> Result<Self>
  where
    I: IntoIterator<Item = (S, Journal)>,
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
      tabs: self.tabs.iter().map(|tab| tab.title().clone()).collect(),
      selected_tab: self.selected_tab,
      date: tab_state.date,
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

  pub fn add_record(&mut self) -> Result<()> {
    self.tab_mut().add_record()
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
