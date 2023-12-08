pub mod journal;
mod level;
mod selectable_list;
mod tab;

use anyhow::Result;
use tokio::sync::mpsc;

use self::{
  journal::Journal, selectable_list::SelectableList, tab::Tab,
};

#[derive(Clone, PartialEq)]
pub struct State {
  pub tabs: SelectableList<String>,
  inner: tab::State,
}

impl std::ops::Deref for State {
  type Target = tab::State;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

pub enum Command {
  NextTab,
  PrevDate,
  NextDate,
  PrevSelection,
  NextSelection,
  AddRecord,
  DeleteSelectedRecord,
  Undo,
  Redo,
  Quit,
  Unknown,
}

pub struct App {
  tabs: Vec<Tab>,
  selected_tab: usize,
  tx: mpsc::Sender<()>,
  rx: mpsc::Receiver<()>,
  should_quit: bool,
}

impl App {
  pub fn try_new<S>(
    journals: Vec<(S, Box<dyn Journal>)>,
  ) -> Result<Self>
  where
    S: ToString,
  {
    let (tx, rx) = mpsc::channel(1);
    let mut app = Self {
      tabs: journals
        .into_iter()
        .map(|(t, j)| Tab::new(t, j, tx.clone()))
        .collect(),
      selected_tab: 0,
      tx,
      rx,
      should_quit: false,
    };
    app.tab_mut().resolve_all()?;
    Ok(app)
  }

  fn tab_mut(&mut self) -> &mut Tab {
    &mut self.tabs[self.selected_tab]
  }

  pub fn state(&self) -> State {
    State {
      tabs: self
        .tabs
        .iter()
        .map(|tab| tab.title().clone())
        .collect::<SelectableList<_>>()
        .with_selected(self.selected_tab),
      inner: self.tab_state().clone(),
    }
  }

  fn tab_state(&self) -> &tab::State {
    self.tabs[self.selected_tab].state()
  }

  pub fn handle_cmd(&mut self, cmd: Command) -> Result<()> {
    use Command::*;
    match cmd {
      Quit => self.should_quit = true,
      NextTab => self.next_tab()?,
      PrevDate => self.tab_mut().prev_date()?,
      NextDate => self.tab_mut().next_date()?,
      PrevSelection => self.tab_mut().prev_selection()?,
      NextSelection => self.tab_mut().next_selection()?,
      AddRecord => self.tab_mut().add_record()?,
      DeleteSelectedRecord => {
        self.tab_mut().delete_selected_record()?
      }
      Undo => self.tab_mut().undo()?,
      Redo => self.tab_mut().redo()?,
      Unknown => (),
    }
    Ok(())
  }

  fn next_tab(&mut self) -> Result<()> {
    self.selected_tab = (self.selected_tab + 1) % self.tabs.len();
    self.tab_mut().resolve_all()?;
    self.tx.try_send(()).unwrap();
    Ok(())
  }

  pub fn changes(&mut self) -> &mut mpsc::Receiver<()> {
    &mut self.rx
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }
}
