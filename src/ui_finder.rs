use std::collections::BTreeMap;
use std::io::{self, stderr};
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, ExecutableCommand};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{self, Style};
use ratatui::widgets::{
  Block, Borders, HighlightSpacing, List, ListDirection, ListState, Paragraph,
};
use ratatui::Frame;
use ratatui::{backend::CrosstermBackend, Terminal};
use unicode_width::UnicodeWidthChar;

use crate::fuzzy;
use crate::path::{self, PathItem};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
struct FilteredKey {
  cost: u32,
  index: usize,
}

struct State {
  // Event loop status
  quit: bool,
  ret: Option<String>,
  need_to_redraw: bool,

  // Fuzzy finder status
  list: Vec<PathItem>,
  query: Vec<char>,

  ed: fuzzy::EditDist,
  unfiltered_count: usize,
  filtered: BTreeMap<FilteredKey, usize>,

  // Editing status
  cursor: usize,
  ui_cursor: usize,
  query_string: String,

  // List status
  list_items: Vec<String>,
  list_state: ListState,
}

impl State {
  fn new(list: Vec<path::PathItem>, init_query: &String) -> Self {
    let cursor = init_query.len();
    let list_items = list.iter().map(|s| s.displayed.clone()).collect();
    let list_state = ListState::default();
    let query: Vec<char> = init_query.chars().collect();
    let unfiltered_count = list.len();
    let ui_cursor = query
      .iter()
      .fold(0, |a, c| a + UnicodeWidthChar::width(*c).unwrap_or(0));
    let mut ed = fuzzy::EditDist::new();
    ed.update_query(&query);
    Self {
      quit: false,
      ret: None,
      need_to_redraw: true,

      list,
      query,

      ed,
      unfiltered_count,
      filtered: BTreeMap::new(),

      cursor,
      ui_cursor,
      query_string: init_query.clone(),

      list_items,
      list_state,
    }
  }

  fn move_cursor(&mut self, mut offset: isize) {
    while offset < 0 {
      if self.cursor <= 0 {
        break;
      }
      self.cursor -= 1;
      self.ui_cursor -=
        UnicodeWidthChar::width(self.query[self.cursor]).unwrap_or(0);
      offset += 1;
    }
    while offset > 0 {
      if self.cursor >= self.query.len() {
        break;
      }
      self.ui_cursor +=
        UnicodeWidthChar::width(self.query[self.cursor]).unwrap_or(0);
      self.cursor += 1;
      offset -= 1;
    }
    self.need_to_redraw = true;
  }

  fn move_selected_item(&mut self, offset: isize) {
    let new_index = self.list_state.selected().unwrap_or(0) as isize + offset;
    self.list_state.select(Some(
      new_index.clamp(0, self.list.len() as isize - 1) as usize,
    ));
    self.need_to_redraw = true;
  }

  fn backspace(&mut self) {
    if self.cursor <= 0 {
      return;
    }
    self.move_cursor(-1);
    self.query.remove(self.cursor);
    self.clear_filtered();
  }

  fn insert(&mut self, new_char: char) {
    self.query.insert(self.cursor, new_char);
    self.move_cursor(1);
    self.clear_filtered();
  }

  fn clear_filtered(&mut self) {
    self.query_string = self.query.iter().collect();
    self.ed.update_query(&self.query);
    self.filtered.clear();
    self.unfiltered_count = self.list.len();
    self.need_to_redraw = true;
  }

  fn filter_slightly(&mut self, duration: Duration) {
    // Get now to check duration
    let now = std::time::Instant::now();
    while now.elapsed() < duration && self.unfiltered_count > 0 {
      let idx = self.unfiltered_count - 1;
      let item = &self.list[idx];

      // Calculate cost
      if let Some(cost) = self.ed.run(&item.displayed) {
        self.filtered.insert(FilteredKey { cost, index: idx }, idx);
      }

      self.unfiltered_count -= 1;
      self.need_to_redraw = true;
    }
  }
}

fn handle_event_ui(s: &mut State, e: Event) {
  match e {
    Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
      KeyCode::Esc => s.quit = true,
      KeyCode::Backspace => s.backspace(),
      KeyCode::Left => s.move_cursor(-1),
      KeyCode::Right => s.move_cursor(1),
      KeyCode::Up => s.move_selected_item(1),
      KeyCode::Down => s.move_selected_item(-1),
      KeyCode::Home => s.move_cursor(-1000),
      KeyCode::End => s.move_cursor(1000),
      KeyCode::Enter => {
        if let Some(selected) = s.list_state.selected() {
          s.ret = s
            .filtered
            .iter()
            .nth(selected)
            .map(|(_, idx)| s.list[*idx].abs.clone());
          s.quit = true;
        }
      }
      KeyCode::Char(to_insert) => {
        if key.modifiers & KeyModifiers::CONTROL != KeyModifiers::empty() {
          match to_insert {
            'n' => s.move_selected_item(-1),
            'p' => s.move_selected_item(1),
            'a' => s.move_cursor(-1000),
            'e' => s.move_cursor(1000),
            _ => s.quit = true,
          }
        } else if key.modifiers & KeyModifiers::ALT != KeyModifiers::empty() {
          match to_insert {
            'j' => s.move_selected_item(-1),
            'k' => s.move_selected_item(1),
            'h' => s.move_cursor(-1),
            'l' => s.move_cursor(1),
            _ => {}
          }
        } else {
          s.insert(to_insert)
        }
      }
      _ => {}
    },
    _ => {}
  }
}

fn draw_ui(f: &mut Frame, s: &mut State) {
  let vertical = Layout::vertical([
    Constraint::Min(1),
    Constraint::Length(1),
    Constraint::Length(1),
  ]);
  let [list_area, bd_area, input_area] = vertical.areas(f.size());

  {
    // Draw input line

    let input_horizontal =
      Layout::horizontal([Constraint::Length(2), Constraint::Min(1)]);
    let [prompt_area, query_area] = input_horizontal.areas(input_area);

    // Draw prompt
    let prompt = Paragraph::new(">").style(Style::default());
    f.render_widget(prompt, prompt_area);

    // Draw query
    let input = Paragraph::new(s.query_string.as_str())
      .style(Style::default())
      .block(Block::default());
    f.render_widget(input, query_area);

    // Move cursor
    f.set_cursor(query_area.x + s.ui_cursor as u16, query_area.y);
  }

  {
    // Draw border
    let mut title = format!(" {}/{} ", s.filtered.len(), s.list.len());
    if s.unfiltered_count > 0 {
      title.push_str(format!("({} left) ", s.unfiltered_count).as_str());
    }
    let block = Block::default().borders(Borders::TOP).title(title);
    f.render_widget(block, bd_area);
  }

  {
    // Draw list items
    let height = list_area.height as usize;

    // Move offset into screen
    let sel = s.list_state.selected().unwrap_or(0);
    s.list_state.select(Some(sel));

    let off = s.list_state.offset_mut();
    *off = (*off).clamp(sel.saturating_sub(height - 1), sel);

    // Render
    let iter = s
      .filtered
      .iter()
      .map(|(_key, val)| s.list_items[*val].as_str());
    let path_list = List::new(iter)
      .direction(ListDirection::BottomToTop)
      .highlight_spacing(HighlightSpacing::Always)
      .highlight_symbol("* ")
      .highlight_style(Style::default().fg(style::Color::Red))
      .block(Block::default());
    f.render_stateful_widget(path_list, list_area, &mut s.list_state);
  }
}

fn run_ui(s: &mut State) -> io::Result<String> {
  // Clean-up UI
  enable_raw_mode()?;
  stderr().execute(EnterAlternateScreen)?;
  let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

  while !s.quit {
    if s.need_to_redraw {
      terminal.draw(|f| draw_ui(f, s))?;
      s.need_to_redraw = false;
    }

    while event::poll(Duration::from_millis(35))? {
      handle_event_ui(s, event::read()?);
    }

    s.filter_slightly(Duration::from_millis(15));
  }

  stderr().execute(LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(s.ret.clone().unwrap_or("".to_string()))
}

pub fn run(list: Vec<path::PathItem>, init_query: &String) -> Option<String> {
  let mut s = State::new(list, init_query);
  run_ui(&mut s).ok().filter(|s| !s.is_empty())
}
