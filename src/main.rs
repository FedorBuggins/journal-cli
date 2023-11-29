mod app;
mod journal;
mod tui;
mod ui;
mod update;

use anyhow::Result;
use app::App;
use ratatui::prelude::{CrosstermBackend, Terminal};
use tui::{event::EventHandler, Tui};
use update::handle_event;

#[tokio::main]
async fn main() -> Result<()> {
  let backend = CrosstermBackend::new(std::io::stderr());
  let terminal = Terminal::new(backend)?;
  let mut tui = Tui::new(terminal, EventHandler::new());
  tui.enter()?;
  let err = run(&mut tui).await.err();
  tui.exit()?;
  if let Some(err) = err {
    eprintln!("{err}");
  }
  Ok(())
}

async fn run(tui: &mut Tui) -> Result<()> {
  let mut app = App::new();
  tui.draw(&app)?;
  while !app.should_quit() {
    handle_event(&mut app, tui.events.next().await?)?;
    tui.draw(&app)?;
  }
  Ok(())
}
