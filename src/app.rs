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
}

pub struct App {
  tabs: SelectableList<Tab>,
  should_quit: bool,
  tx: mpsc::Sender<()>,
  rx: mpsc::Receiver<()>,
}

impl App {
  pub fn new<I, S>(journals: I) -> Self
  where
    I: IntoIterator<Item = (S, Box<dyn Journal>)>,
    S: ToString,
  {
    let (tx, rx) = mpsc::channel(1);
    let tabs = journals
      .into_iter()
      .map(|(t, j)| Tab::new(t, j, tx.clone()))
      .collect();
    Self {
      tabs,
      should_quit: false,
      tx,
      rx,
    }
  }

  pub fn init(mut self) -> Result<Self> {
    self.tab_mut().resolve_all()?;
    Ok(self)
  }

  fn tab_mut(&mut self) -> &mut Tab {
    self.tabs.selected_item_mut().unwrap()
  }

  pub fn state(&self) -> State {
    State {
      tabs: self.tabs.map(|tab| tab.title().clone()),
      inner: self.tabs.selected_item().unwrap().state().clone(),
    }
  }

  pub fn handle_cmd(&mut self, cmd: Command) -> Result<()> {
    use Command::*;
    match cmd {
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
      Quit => self.should_quit = true,
    }
    Ok(())
  }

  fn next_tab(&mut self) -> Result<()> {
    self.tabs.wrapping_select_next();
    self.tab_mut().resolve_all()?;
    self.tx.try_send(())?;
    Ok(())
  }

  pub fn changes(&mut self) -> &mut mpsc::Receiver<()> {
    &mut self.rx
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }
}
