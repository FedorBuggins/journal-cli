use std::cmp::max;

use chrono::{Datelike, Local, Month, NaiveDate, Timelike, Weekday};
use ratatui::{
  prelude::{Buffer, Constraint, Direction, Rect},
  style::{Color, Stylize},
  text::{Line, Span},
  widgets::{
    Bar, BarChart, BarGroup, Block, List, ListItem, Paragraph, Tabs,
    Widget,
  },
};

use crate::app::State;

use super::{layout, styles};

pub fn tabs(state: &State) -> Tabs<'_> {
  let titles = state
    .tabs
    .iter()
    .map(|title| format!("[ {title} ]"))
    .collect();
  Tabs::new(titles).divider("").select(state.tabs.selected())
}

pub fn date_paragraph<'a>(date: NaiveDate) -> Paragraph<'a> {
  Paragraph::new(format!("< {} >", date.format("%a, %-d %b %Y")))
}

pub fn record_list(state: &State) -> List<'_> {
  let items: Vec<_> = state
    .list
    .iter()
    .enumerate()
    .map(|(i, dt)| {
      let text = format!("{}) {}", i + 1, dt.format("%R"));
      ListItem::new(text).style(if i == state.list.selected() {
        styles::ACCENT.reversed()
      } else {
        styles::PRIMARY
      })
    })
    .collect();
  List::new(items)
}

pub fn level_bar(state: &State) -> BarChart<'_> {
  let percentage = state.level.percentage();
  let level = (percentage * 100.).round() as _;
  let count = state.level.count();
  let target = state.level.target();
  BarChart::default()
    .bar_width(13)
    .max(bar_max(max(level, 100)))
    .data(
      BarGroup::default().bars(&[Bar::default()
        .value(level)
        .text_value(format!("{count}/{target} ({level}%)"))]),
    )
    .bar_style(
      styles::RED.fg(level_color(percentage, state.tabs.selected())),
    )
}

fn level_color(mut percentage: f32, tab_index: usize) -> Color {
  const RGB_MAX: f32 = u8::MAX as f32;
  const K: f32 = 2.5;
  if tab_index == 0 {
    percentage = (1. - percentage).abs();
  }
  Color::Rgb(
    (RGB_MAX * percentage * K) as _,
    (RGB_MAX * (1. - percentage.powf(K))) as _,
    (RGB_MAX * (1. - percentage * K)) as _,
  )
}

pub struct DaysBarChart<'a> {
  state: &'a State,
  block: Option<Block<'a>>,
}

impl<'a> DaysBarChart<'a> {
  const GAP: u16 = 2;

  pub fn new(state: &'a State) -> Self {
    Self { state, block: None }
  }

  pub fn block(mut self, block: Block<'a>) -> Self {
    self.block = Some(block);
    self
  }

  fn render_block(&mut self, area: &mut Rect, buf: &mut Buffer) {
    if let Some(block) = self.block.take() {
      let inner = block.inner(*area);
      block.render(*area, buf);
      *area = inner;
    }
  }

  fn bar_chart(&self) -> BarChart<'a> {
    let bars: Vec<_> = self
      .state
      .recs_by_date
      .iter()
      .map(|&(date, count)| {
        let label = date.format("%e").to_string();
        if date == self.state.date {
          Bar::default()
            .label(Line::styled(label, styles::ACCENT))
            .value(count as _)
            .style(styles::ACCENT)
        } else {
          Bar::default()
            .label(Line::styled(label, styles::PRIMARY))
            .value(count as _)
            .style(styles::PRIMARY.fg(level_color(
              count as f32 / self.state.level.middle(),
              self.state.tabs.selected(),
            )))
        }
      })
      .collect();

    let max_val = self
      .state
      .recs_by_date
      .iter()
      .map(|(_, count)| count)
      .max()
      .map_or(0, |v| *v);

    BarChart::default()
      .bar_width(2)
      .bar_gap(Self::GAP)
      .max(bar_max(max_val as _))
      .data(BarGroup::default().bars(&bars))
  }

  fn weekdays_paragraph(&self) -> Paragraph<'a> {
    let space = " ".repeat(Self::GAP as _);
    let weekdays: Vec<_> = self
      .state
      .recs_by_date
      .iter()
      .map(|(date, _)| {
        let weekday = weekday(date);
        let weekday_symbols = &weekday.to_string()[..2];
        let style = if date == &self.state.date {
          styles::ACCENT
        } else if weekday == Weekday::Sun {
          styles::RED
        } else {
          styles::PRIMARY
        };
        Span::styled(format!("{weekday_symbols}{space}"), style)
      })
      .collect();
    Paragraph::new(Line::from(weekdays))
  }
}

impl<'a> Widget for DaysBarChart<'a> {
  fn render(mut self, mut area: Rect, buf: &mut Buffer) {
    use Constraint::*;

    self.render_block(&mut area, buf);
    let [chart, weekdays] = layout::vsplit([Min(5), Length(1)], area);
    self.bar_chart().render(chart, buf);
    self.weekdays_paragraph().render(weekdays, buf);
  }
}

fn weekday(date: &NaiveDate) -> Weekday {
  date.and_time(Default::default()).weekday()
}

fn bar_max(max_val: u64) -> u64 {
  const INCREASE_PERCENTAGE: u64 = 10;
  max_val + max(max_val / INCREASE_PERCENTAGE, 1)
}

pub fn time_smoke_records_bar_chart(state: &State) -> BarChart<'_> {
  const HOURS_COUNT: u8 = 24;

  let bars: Vec<_> = (0..HOURS_COUNT)
    .map(|h| {
      let label = format!("{:0>2}:00", h);
      let value = state.recs_by_hour.get(&h).map_or(0, |v| *v);
      Bar::default()
        .label(label.into())
        .value(value as _)
        .text_value(String::new())
        .style(
          if state
            .list
            .selected_item()
            .is_some_and(|dt| h == dt.time().hour() as _)
          {
            styles::ACCENT
          } else {
            styles::RED
          },
        )
    })
    .collect();

  BarChart::default()
    .direction(Direction::Horizontal)
    .bar_gap(0)
    .max(3)
    .data(BarGroup::default().bars(&bars))
}

pub fn year_smoke_records_bar_chart(state: &State) -> BarChart<'_> {
  let today = Local::now().date_naive();
  let cur_month = Month::try_from(today.month0() as u8 + 1).unwrap();
  let past_year = iter_months(cur_month).skip(1).take(12);
  let bars: Vec<_> = past_year
    .map(|month| {
      let label = month.name()[0..3].to_string();
      let value = state.recs_by_month.get(&month).map_or(0, |v| *v);
      Bar::default().label(label.into()).value(value as _)
    })
    .collect();

  let max_val = state.recs_by_month.values().max().map_or(0, |v| *v);

  BarChart::default()
    .bar_width(3)
    .bar_gap(1)
    .max(bar_max(max_val as _))
    .data(BarGroup::default().bars(&bars))
}

fn iter_months(start: Month) -> impl Iterator<Item = Month> {
  (0..).scan(start, |next, _| {
    let cur = *next;
    *next = cur.succ();
    Some(cur)
  })
}

pub fn help_paragraph<'a>() -> Paragraph<'a> {
  Paragraph::new("SPACE - add record, u - undo, U - redo, ESC - quit")
}
