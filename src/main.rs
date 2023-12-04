mod app;
mod journal;
mod tui;
mod ui;
mod update;

use anyhow::Result;
use app::App;
use tui::Tui;
use update::handle_event;

#[tokio::main]
async fn main() -> Result<()> {
  let mut tui = Tui::try_new()?;
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
  while !app.should_quit() {
    tui.draw(&app)?;
    handle_event(&mut app, tui.event().await?)?;
  }
  Ok(())
}
