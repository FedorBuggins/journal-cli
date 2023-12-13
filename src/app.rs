pub mod journal;
pub mod level;
mod selectable_list;
mod tab;

use std::sync::Arc;

use anyhow::{Context, Result};
use futures::{Future, FutureExt};
use tokio::{
  sync::{mpsc, watch, Mutex},
  task::{AbortHandle, JoinHandle},
};

use self::{
  journal::Journal, selectable_list::SelectableList, tab::Tab,
};

#[derive(Default, Clone, PartialEq)]
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

impl State {
  pub fn new(tabs: impl IntoIterator<Item = String>) -> Self {
    Self {
      tabs: tabs.into_iter().collect(),
      ..Default::default()
    }
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
  tabs: SelectableList<Arc<Mutex<Tab>>>,
  state_tx: Arc<Mutex<watch::Sender<State>>>,
  state_rx: watch::Receiver<State>,
  changes_rx: mpsc::Receiver<()>,
  abort_handle: AbortHandle,
  should_quit: bool,
}

impl App {
  pub fn new<I, S>(args: I) -> Self
  where
    I: IntoIterator<Item = (S, usize, Box<dyn Journal>)>,
    S: ToString,
  {
    let tabs: Vec<_> = args
      .into_iter()
      .map(|(t, c, j)| Tab::new(t, c, j))
      .collect();
    let tab_titles = tabs.iter().map(|tab| tab.title().clone());
    let (state_tx, state_rx) = watch::channel(State::new(tab_titles));
    let state_tx = Arc::new(Mutex::new(state_tx));
    let (changes_tx, changes_rx) = mpsc::channel(1);

    Self::subscribe_on_tabs(&tabs, state_tx.clone());
    Self::broadcast_changes(state_rx.clone(), changes_tx);

    let tabs =
      tabs.into_iter().map(|t| Arc::new(Mutex::new(t))).collect();
    let abort_handle =
      tokio::spawn(async { Ok(()) as Result<()> }).abort_handle();

    Self {
      tabs,
      state_tx,
      state_rx,
      changes_rx,
      abort_handle,
      should_quit: false,
    }
  }

  fn subscribe_on_tabs(
    tabs: &[Tab],
    state_tx: Arc<Mutex<watch::Sender<State>>>,
  ) {
    let mut active_tab = 0;
    let mut tab_states: Vec<_> =
      tabs.iter().map(|tab| tab.subscribe()).collect();
    tokio::spawn(async move {
      let mut state_rx = state_tx.lock().await.subscribe();
      loop {
        let tab_changed = tab_states[active_tab].changed();
        let tab_switched = state_rx
          .wait_for(|state| state.tabs.selected() != active_tab)
          .map(|state| state.unwrap().tabs.selected());
        tokio::select! {
          _ = tab_changed => (),
          selected_tab = tab_switched => {
            active_tab = selected_tab;
            continue;
          },
        }
        let active_tab_state =
          tab_states[active_tab].borrow().clone();
        let state_tx = state_tx.lock().await;
        state_tx.send_modify(|state| state.inner = active_tab_state);
      }
    });
  }

  fn broadcast_changes(
    mut state_rx: watch::Receiver<State>,
    changes_tx: mpsc::Sender<()>,
  ) {
    tokio::spawn(async move {
      loop {
        state_rx.changed().await.unwrap();
        changes_tx.send(()).await.unwrap();
      }
    });
  }

  pub async fn init(mut self) -> Result<Self> {
    self.spawn_tab_abortable(|tab| async move {
      tab.lock().await.resolve_all().await
    });
    Ok(self)
  }

  pub fn state(&self) -> State {
    self.state_rx.borrow().clone()
  }

  pub fn handle_cmd(&mut self, cmd: Command) -> Result<()> {
    use Command::*;
    match cmd {
      NextTab => self.next_tab()?,
      PrevDate => self.spawn_tab_abortable(|tab| async move {
        tab.lock().await.prev_date().await
      }),
      NextDate => self.spawn_tab_abortable(|tab| async move {
        tab.lock().await.next_date().await
      }),
      PrevSelection => self.spawn_tab_abortable(|tab| async move {
        tab.lock().await.prev_selection().await
      }),
      NextSelection => self.spawn_tab_abortable(|tab| async move {
        tab.lock().await.next_selection().await
      }),
      AddRecord => self.spawn_tab_blocking(|tab| async move {
        tab.lock().await.add_record().await
      }),
      DeleteSelectedRecord => {
        self.spawn_tab_blocking(|tab| async move {
          tab.lock().await.delete_selected_record().await
        })
      }
      Undo => self.spawn_tab_blocking(|tab| async move {
        tab.lock().await.undo().await
      }),
      Redo => self.spawn_tab_blocking(|tab| async move {
        tab.lock().await.redo().await
      }),
      Quit => self.should_quit = true,
    }
    Ok(())
  }

  fn next_tab(&mut self) -> Result<()> {
    self.tabs.wrapping_select_next();
    self
      .state_tx
      .try_lock()
      .context("use lock?!")?
      .send_modify(|state| state.tabs.select(self.tabs.selected()));
    self.spawn_tab_abortable(|tab| async move {
      tab.lock().await.resolve_all().await
    });
    Ok(())
  }

  fn spawn_tab_abortable<F>(
    &mut self,
    f: impl Fn(Arc<Mutex<Tab>>) -> F + Send + 'static,
  ) where
    F: Future<Output = Result<()>> + Send,
  {
    self.abort_handle.abort();
    self.abort_handle = self.spawn_tab(f).abort_handle();
  }

  fn spawn_tab<F>(
    &self,
    f: impl Fn(Arc<Mutex<Tab>>) -> F + Send + 'static,
  ) -> JoinHandle<()>
  where
    F: Future<Output = Result<()>> + Send,
  {
    let tab = self.tabs.selected_item().unwrap().clone();
    tokio::spawn(async move { f(tab).await.unwrap() })
  }

  fn spawn_tab_blocking<F>(
    &self,
    f: impl Fn(Arc<Mutex<Tab>>) -> F + Send + 'static,
  ) where
    F: Future<Output = Result<()>> + Send,
  {
    self.abort_handle.abort();
    self.spawn_tab(f);
  }

  pub fn changes(&mut self) -> &mut mpsc::Receiver<()> {
    &mut self.changes_rx
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }
}
