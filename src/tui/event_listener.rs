use anyhow::{Context, Result};
use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

type AppEvent = super::Event;
type Receiver<T> = mpsc::UnboundedReceiver<T>;
type Sender<T> = mpsc::UnboundedSender<T>;

#[derive(Debug)]
pub struct EventListener {
  rx: Receiver<AppEvent>,
}

impl EventListener {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(async move { listen_event_stream(&tx).await });
    Self { rx }
  }

  pub async fn next(&mut self) -> Result<AppEvent> {
    self
      .rx
      .recv()
      .await
      .context("unable to get event")
      .context("stream closed")
  }
}

async fn listen_event_stream(tx: &Sender<AppEvent>) -> Result<()> {
  let mut reader = EventStream::new();
  loop {
    if let Some(event) = reader.next().fuse().await {
      handle_crossterm_event(event.context("io error"), tx)?;
    }
  }
}

fn handle_crossterm_event(
  event: Result<Event>,
  tx: &Sender<AppEvent>,
) -> Result<()> {
  match event {
    Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
      tx.send(AppEvent::KeyPress(key))?
    }
    Ok(Event::Resize(_, _)) => tx.send(AppEvent::Resize)?,
    Err(error) => tx.send(AppEvent::Error(error.to_string()))?,
    _ => (),
  };
  Ok(())
}
