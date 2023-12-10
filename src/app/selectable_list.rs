use std::cmp::min;

#[derive(Default, Clone, PartialEq)]
pub struct SelectableList<T> {
  items: Vec<T>,
  selected: usize,
  reversed_selection: bool,
}

impl<T> SelectableList<T> {
  pub fn new() -> Self {
    Self {
      items: vec![],
      selected: 0,
      reversed_selection: false,
    }
  }

  pub fn with_reversed_selection(mut self, val: bool) -> Self {
    self.reversed_selection = val;
    self
  }

  pub fn selected(&self) -> usize {
    let index = self._selected();
    if self.reversed_selection {
      self.last_index().saturating_sub(index)
    } else {
      index
    }
  }

  fn _selected(&self) -> usize {
    min(self.selected, self.len().saturating_sub(1))
  }

  fn last_index(&self) -> usize {
    self.len().saturating_sub(1)
  }

  pub fn selected_item(&self) -> Option<&T> {
    self.get(self.selected())
  }

  pub fn select(&mut self, selected: usize) {
    self.selected = min(selected, self.last_index());
  }

  pub fn select_prev(&mut self) {
    if self.reversed_selection {
      self._select_next();
    } else {
      self._select_prev();
    }
  }

  fn _select_prev(&mut self) {
    self.select(self._selected().saturating_sub(1));
  }

  pub fn select_next(&mut self) {
    if self.reversed_selection {
      self._select_prev();
    } else {
      self._select_next();
    }
  }

  fn _select_next(&mut self) {
    self.select(self._selected().saturating_add(1));
  }

  pub fn wrapping_select_next(&mut self) {
    self.selected = self.selected.wrapping_add(1) % self.len();
  }
}

impl<T> std::ops::Deref for SelectableList<T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    &self.items
  }
}

impl<T> std::ops::DerefMut for SelectableList<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.items
  }
}

impl<T> FromIterator<T> for SelectableList<T> {
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = T>,
  {
    Self {
      items: iter.into_iter().collect(),
      ..Self::new()
    }
  }
}
