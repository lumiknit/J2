use std::io::{self, stderr};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
};
use crossterm::{event, ExecutableCommand};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;
use ratatui::{backend::CrosstermBackend, Terminal};

struct State {
  quit: bool,
  list: Vec<String>,
  query: String,
  cursor: usize,
}

impl State {
  fn new(list: Vec<String>, init_query: String) -> Self {
    let cursor = init_query.len();
    Self {
      quit: false,
      list,
      query: init_query,
      cursor,
    }
  }

  fn move_cursor(&mut self, offset: isize) {
    let new_cursor = self.cursor as isize + offset;
    self.cursor = new_cursor.clamp(0, self.query.len() as isize) as usize;
  }

  fn backspace(&mut self) {
    if self.cursor <= 0 {
      return;
    }
    let before_cursor = self.query.chars().take(self.cursor - 1);
    let after_cursor = self.query.chars().skip(self.cursor);
    self.query = before_cursor.chain(after_cursor).collect();
    self.move_cursor(-1);
  }

  fn insert(&mut self, new_char: char) {
    self.query.insert(self.cursor, new_char);
    self.move_cursor(1);
  }
}

fn handle_event_ui(s: &mut State, e: Event) {
  match e {
    Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
      KeyCode::Esc => s.quit = true,
      KeyCode::Backspace => s.backspace(),
      KeyCode::Left => s.move_cursor(-1),
      KeyCode::Right => s.move_cursor(1),
      KeyCode::Char(to_insert) => s.insert(to_insert),
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

  let input_horizontal =
    Layout::horizontal([Constraint::Length(2), Constraint::Min(1)]);
  let [prompt_area, query_area] = input_horizontal.areas(input_area);

  // Draw prompt
  let prompt = Paragraph::new(">").style(Style::default());
  f.render_widget(prompt, prompt_area);

  // Draw query
  let input = Paragraph::new(s.query.as_str())
    .style(Style::default())
    .block(Block::default());
  f.render_widget(input, query_area);

  // Move cursor
  f.set_cursor(query_area.x + s.cursor as u16, query_area.y);

  // Draw border
  let block = Block::default().borders(Borders::TOP).title("Count");
  f.render_widget(block, bd_area);

  // Draw items
  let paths: Vec<ListItem> = s
    .list
    .iter()
    .take(list_area.height as usize)
    .map(|s| ListItem::new(s.as_str()))
    .collect();
  let path_list = List::new(paths).block(Block::default());
  f.render_widget(path_list, list_area);
}

fn run_ui(s: &mut State) -> io::Result<String> {
  // Clean-up UI
  enable_raw_mode()?;
  stderr().execute(EnterAlternateScreen)?;
  let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

  while !s.quit {
    terminal.draw(|f| draw_ui(f, s))?;

    handle_event_ui(s, event::read()?);
  }

  disable_raw_mode()?;
  Ok("".to_string())
}

pub fn run(list: Vec<String>, init_query: String) -> Option<String> {
  let mut s = State::new(list, init_query);
  run_ui(&mut s).ok().filter(|s| !s.is_empty())
}
