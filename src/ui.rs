mod layout;
mod styles;
mod widgets;

use ratatui::{
  prelude::{Alignment, Frame},
  style::Stylize,
  widgets::{Block, BorderType, Borders, Padding},
};

use crate::app::State;

use self::layout::Body;

const CARD: Block<'_> = Block::new()
  .padding(Padding::horizontal(1))
  .borders(Borders::ALL)
  .border_type(BorderType::Rounded)
  .border_style(styles::SECONDARY);

pub fn render(state: &State, f: &mut Frame) {
  let Body {
    tabs,
    date,
    days,
    time,
    year,
    help,
  } = Body::new(f.size());

  f.render_widget(
    widgets::tabs(state)
      .style(styles::PRIMARY)
      .highlight_style(styles::SECONDARY.bold().underlined()),
    tabs,
  );

  f.render_widget(
    widgets::date_paragraph(state.date)
      .style(styles::ACCENT)
      .alignment(Alignment::Right),
    date,
  );

  f.render_widget(
    widgets::date_records_bar_chart(state)
      .block(CARD.title(st("Days")))
      .style(styles::PRIMARY),
    days,
  );

  f.render_widget(
    widgets::time_smoke_records_bar_chart(state)
      .block(CARD.title(st("Time")))
      .style(styles::PRIMARY),
    time,
  );

  f.render_widget(
    widgets::year_smoke_records_bar_chart(state)
      .block(CARD.title(st("Months")).padding(Padding::horizontal(4)))
      .style(styles::PRIMARY),
    year,
  );

  f.render_widget(
    widgets::help_paragraph()
      .style(styles::GREY)
      .alignment(Alignment::Center),
    help,
  );
}

/// Style title
fn st(title: &str) -> String {
  format!("| {title} |")
}
