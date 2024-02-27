use anyhow::{Context, Result};
use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{self, error::SendError};

type TuiEvent = super::Event;
type Receiver<T> = mpsc::UnboundedReceiver<T>;
type Sender<T> = mpsc::UnboundedSender<T>;

#[derive(Debug)]
pub struct EventListener {
  rx: Receiver<TuiEvent>,
}

impl EventListener {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
      if let Err(err) = listen_event_stream(&tx).await {
        tx.send(TuiEvent::Error(err.to_string()))
          .expect("can't send tui event");
      }
    });
    Self { rx }
  }

  pub async fn next(&mut self) -> Result<TuiEvent> {
    self
      .rx
      .recv()
      .await
      .context("crossterm events stream closed")
  }
}

async fn listen_event_stream(
  tx: &Sender<TuiEvent>,
) -> Result<(), SendError<TuiEvent>> {
  let mut reader = EventStream::new();
  loop {
    if let Some(event) = reader.next().fuse().await {
      handle_crossterm_event(event.context("io error"), tx)?;
    }
  }
}

fn handle_crossterm_event(
  event: Result<Event>,
  tx: &Sender<TuiEvent>,
) -> Result<(), SendError<TuiEvent>> {
  match event {
    Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
      tx.send(TuiEvent::KeyPress(key))?;
    }
    Ok(Event::Resize(_, _)) => {
      tx.send(TuiEvent::Resize)?;
    }
    Err(error) => {
      tx.send(TuiEvent::Error(error.to_string()))?;
    }
    _ => (),
  }
  Ok(())
}
