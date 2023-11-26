use std::collections::HashMap;

use chrono::{Days, Local, NaiveDate, Timelike};
use ratatui::{
  prelude::{Alignment, Constraint, Direction, Frame, Layout},
  style::{Color, Style},
  text::Line,
  widgets::{
    Bar, BarChart, BarGroup, Block, BorderType, Borders, Padding,
    Paragraph,
  },
};

use crate::app::App;

pub fn render(app: &App, f: &mut Frame) {
  let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Length(4),
      Constraint::Length(2),
      Constraint::Max(10),
      Constraint::Length(1),
      Constraint::Min(15),
    ])
    .split(f.size());

  let [header, date, day, _, dates] =
    [0, 1, 2, 3, 4].map(|i| layout[i]);

  f.render_widget(
    help_paragraph()
      .block(
        Block::default()
          .title("Smoke Journal")
          .title_alignment(Alignment::Center)
          .title_style(Style::default().fg(Color::Yellow))
          .padding(Padding::uniform(1)),
      )
      .style(Style::default().fg(Color::DarkGray))
      .alignment(Alignment::Center),
    header,
  );

  f.render_widget(
    date_paragraph(app.date())
      .style(Style::default().fg(Color::Green))
      .alignment(Alignment::Center),
    date,
  );

  f.render_widget(
    today_smoke_records_bar_chart(app)
      .block(
        Block::default()
          .title("Smokes by hours")
          .padding(Padding::horizontal(1))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    day,
  );

  f.render_widget(
    two_weeks_smoke_records_bar_chart(app)
      .block(
        Block::default()
          .title("Smokes by dates")
          .padding(Padding::horizontal(1))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    dates,
  )
}

fn date_paragraph<'a>(date: NaiveDate) -> Paragraph<'a> {
  Paragraph::new(format!("<- {} ->", date.format("%b %-d")))
}

fn help_paragraph<'a>() -> Paragraph<'a> {
  Paragraph::new(
    "Press `s` to add smoke record, `q` to stop running.",
  )
}

fn today_smoke_records_bar_chart(app: &App) -> BarChart<'_> {
  let rec_map_by_hour = app.date_smoke_records().into_iter().fold(
    HashMap::new(),
    |mut acc, dt| {
      let h = dt.time().hour();
      *acc.entry(h).or_default() += 1;
      acc
    },
  );

  BarChart::default().bar_width(2).bar_gap(2).max(3).data(
    BarGroup::default().bars(
      &(0..12)
        .map(|mut h| {
          h *= 2;
          Bar::default().label(Line::raw(h.to_string())).value(
            (h..=h + 1).flat_map(|h| rec_map_by_hour.get(&h)).sum(),
          )
        })
        .collect::<Vec<_>>(),
    ),
  )
}

fn two_weeks_smoke_records_bar_chart(app: &App) -> BarChart<'_> {
  let today = Local::now().date_naive();
  BarChart::default().bar_width(2).bar_gap(2).max(15).data(
    BarGroup::default().bars(
      &app
        .smoke_records_for(today - Days::new(13), today)
        .map(|(date, recs)| {
          let color = if date == app.date() {
            Color::Green
          } else {
            Color::Yellow
          };

          Bar::default()
            .label(Line::raw(date.format("%d").to_string()))
            .value(recs.len() as _)
            .style(Style::default().fg(color))
        })
        .collect::<Vec<_>>(),
    ),
  )
}
