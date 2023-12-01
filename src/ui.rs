mod layout;
mod styles;
mod widgets;

use ratatui::{
  prelude::{Alignment, Frame},
  style::Stylize,
  widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::app::State;

use self::layout::Body;

const CARD: Block<'_> = Block::new()
  .padding(Padding::horizontal(1))
  .borders(Borders::ALL)
  .border_type(BorderType::Rounded);

pub fn render(state: &State, f: &mut Frame) {
  let Body {
    title,
    date,
    days,
    time,
    year,
    help,
  } = Body::new(f.size());

  f.render_widget(
    Paragraph::new("Smoker Journal")
      .alignment(Alignment::Center)
      .style(styles::PRIMARY.bold()),
    title,
  );

  f.render_widget(
    widgets::date_paragraph(state.date)
      .style(styles::ACCENT.bold())
      .alignment(Alignment::Center),
    date,
  );

  f.render_widget(
    widgets::date_smoke_records_bar_chart(state)
      .block(CARD.title("Days"))
      .style(styles::PRIMARY),
    days,
  );

  f.render_widget(
    widgets::time_smoke_records_bar_chart(state)
      .block(CARD.title("Time"))
      .style(styles::PRIMARY),
    time,
  );

  f.render_widget(
    widgets::year_smoke_records_bar_chart(state)
      .block(CARD.title("Months").padding(Padding::horizontal(4)))
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
