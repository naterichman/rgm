use crate::screen::Draw;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    terminal::Frame,
    widgets::{Block, Paragraph},
};
pub enum InputStatus {
    Error,
    Info,
    Warning,
}

pub struct Input {
    editing: bool,
    text: String,
    status: InputStatus,
}

impl Input {
    pub fn error(msg: String) -> Self {
        Self {
            editing: false,
            text: msg,
            status: InputStatus::Error,
        }
    }

    pub fn warning(msg: String) -> Self {
        Self {
            editing: false,
            text: msg,
            status: InputStatus::Warning,
        }
    }

    pub fn push(&mut self, v: char) {
        self.text.push(v);
    }

    pub fn pop(&mut self) {
        self.text.pop();
    }

    pub fn editing(&mut self, v: bool) {
        self.editing = v;
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}

impl Draw for Input {
    fn draw<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let input = Paragraph::new(self.text.as_ref())
            .block(Block::default())
            .style(match self.status {
                InputStatus::Info => Style::default(),
                InputStatus::Warning => Style::default().fg(Color::Yellow),
                InputStatus::Error => Style::default().fg(Color::Red),
            });
        frame.render_widget(input, area);
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            editing: false,
            text: String::from(""),
            status: InputStatus::Info,
        }
    }
}
