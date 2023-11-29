mod layout;
mod widgets;

use ratatui::{
  prelude::{Alignment, Frame},
  style::{Color, Modifier, Style},
  widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::app::State;

use self::layout::Body;

pub fn render(state: &State, f: &mut Frame) {
  let Body {
    title,
    date,
    dates,
    time,
    year,
    help,
  } = Body::new(f.size());

  f.render_widget(
    Paragraph::new("Smoker Journal")
      .alignment(Alignment::Center)
      .style(
        Style::default()
          .fg(Color::Yellow)
          .add_modifier(Modifier::BOLD),
      ),
    title,
  );

  f.render_widget(
    widgets::date_paragraph(state.date)
      .style(
        Style::default()
          .fg(Color::Green)
          .add_modifier(Modifier::BOLD),
      )
      .alignment(Alignment::Center),
    date,
  );

  f.render_widget(
    widgets::date_smoke_records_bar_chart(state)
      .block(
        Block::default()
          .title("Date")
          .padding(Padding::horizontal(1))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    dates,
  );

  f.render_widget(
    widgets::time_smoke_records_bar_chart(state)
      .block(
        Block::default()
          .title("Time")
          .padding(Padding::horizontal(1))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    time,
  );

  f.render_widget(
    widgets::year_smoke_records_bar_chart(state)
      .block(
        Block::default()
          .title("Year")
          .padding(Padding::horizontal(4))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    year,
  );

  f.render_widget(
    widgets::help_paragraph()
      .style(Style::default().fg(Color::DarkGray))
      .alignment(Alignment::Center),
    help,
  );
}
