mod tab;

use std::{collections::HashMap, path::Path};

use chrono::{Month, NaiveDate};

use crate::journal::Journal;

use self::tab::Tab;

const ROOT: &str = concat!(env!("HOME"), "/.journals");

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
  pub fn new() -> Self {
    let root = Path::new(ROOT);
    let trains_journal = Journal::new(root.join("trains"));
    let smokes_journal = Journal::new(root.join("smokes"));
    Self {
      tabs: vec![
        Tab::new("Trains", trains_journal),
        Tab::new("Smokes", smokes_journal),
      ],
      selected_tab: 0,
      should_quit: false,
    }
    .init()
  }

  fn init(mut self) -> Self {
    self.resolve_all();
    self
  }

  fn resolve_all(&mut self) {
    self.tab_mut().resolve_all();
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

  pub fn next_tab(&mut self) {
    self.selected_tab = (self.selected_tab + 1) % self.tabs.len();
    self.resolve_all();
  }

  pub fn prev_date(&mut self) {
    self.tab_mut().prev_date();
  }

  pub fn next_date(&mut self) {
    self.tab_mut().next_date();
  }

  pub fn add_record(&mut self) {
    self.tab_mut().add_record();
  }

  pub fn undo(&mut self) {
    self.tab_mut().undo();
  }

  pub fn redo(&mut self) {
    self.tab_mut().redo();
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }

  pub fn quit(&mut self) {
    self.should_quit = true;
  }
}
