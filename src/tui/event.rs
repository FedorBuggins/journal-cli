use anyhow::{Context, Result};
use crossterm::event::{EventStream, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

type CrosstermEvent = crossterm::event::Event;

#[derive(Clone, Debug)]
pub enum Event {
  KeyPress(KeyEvent),
  Resize,
  Error(String),
}

#[derive(Debug)]
pub struct EventHandler {
  rx: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
      let mut reader = EventStream::new();
      loop {
        if let Some(event) = reader.next().fuse().await {
          handle_crossterm_event(event.context("io error"), &tx);
        }
      }
    });

    Self { rx }
  }

  pub async fn next(&mut self) -> Result<Event> {
    self.rx.recv().await.context("unable to get event")
  }
}

fn handle_crossterm_event(
  event: Result<CrosstermEvent>,
  tx: &mpsc::UnboundedSender<Event>,
) {
  match event {
    Ok(CrosstermEvent::Key(key))
      if key.kind == KeyEventKind::Press =>
    {
      tx.send(Event::KeyPress(key)).unwrap()
    }
    Ok(CrosstermEvent::Resize(_, _)) => {
      tx.send(Event::Resize).unwrap()
    }
    Err(error) => tx.send(Event::Error(error.to_string())).unwrap(),
    _ => (),
  }
}
