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
    Warning,
    Info,
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
        // Leave command prompt
        if self.text.len() > 1 {
            self.text.pop();
        }
    }

    pub fn editing(&mut self, v: bool) {
        self.editing = v;
        if v {
            // Set command prompt to be `:`
            self.text = String::from(":");
        }
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn clear(&mut self) {
        self.text = String::new()
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            editing: false,
            text: String::new(),
            status: InputStatus::Info,
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn input_editing() {
        let mut def = super::Input::default();
        assert_eq!(
            def,
            super::Input {
                editing: false,
                text: String::new(),
                status: super::InputStatus::Info
            }
        );
        def.editing(true);
        def.push('a');
        def.push('b');
        assert_eq!(def.is_editing(), true);
        assert_eq!(def.text(), String::from(":ab"));
        // pop doesn't remove `:` prompt
        def.pop();
        def.pop();
        def.pop();
        def.editing(false);
        assert_eq!(def.is_editing(), false);
        assert_eq!(def.text(), String::from(":"));
        // Clear removes text
        def.clear();
        assert_eq!(def.text(), String::new());
    }
}
