use crate::input::Input;
use crate::repo::Repos;
use crate::repoview::RepoView;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{debug, info};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    terminal::{Frame, Terminal},
    widgets::{Block, Paragraph},
};

pub trait Draw {
    fn draw<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect);
}

pub struct Screen {
    repoview: RepoView,
    input: Input,
}

impl Screen {
    pub fn new(repos: Repos) -> Self {
        let longest = repos.longest_name();
        let repoview = RepoView::new(repos);
        Self {
            repoview,
            input: Input::default(),
        }
    }

    fn get_terminal<W>(mut writer: W) -> Result<Terminal<CrosstermBackend<W>>, Box<dyn Error>>
    where
        W: io::Write,
    {
        enable_raw_mode()?;
        execute!(writer, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(writer);
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }

    pub fn run<W>(&mut self, writer: W) -> Result<(), Box<dyn Error>>
    where
        W: io::Write,
    {
        let mut terminal = Screen::get_terminal(writer)?;
        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        loop {
            self.draw(&mut terminal);
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                let should_exit = self.handle_event();
                if should_exit {
                    break;
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        self.exit(&mut terminal)?;
        Ok(())
    }

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> bool {
        let res = terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(&size.height - 1), Constraint::Length(1)].as_ref())
                .split(size);

            // Render list of repos
            {
                self.repoview.draw(f, chunks[0]);
            }
            // Render input
            {
                self.input.draw(f, chunks[1]);
            }
        });
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn exit<B>(&self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>>
    where
        B: Backend,
    {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn handle_event(&mut self) -> bool {
        let raw_evt = match event::read() {
            Ok(evt) => evt,
            Err(e) => {
                println!("{}", e.to_string());
                return true;
            }
        };

        if let Event::Key(key) = raw_evt {
            if self.input.is_editing() {
                match key.code {
                    KeyCode::Char(x) => self.input.push(x),
                    KeyCode::Enter => self.parse_command(),
                    KeyCode::Backspace => {
                        self.input.pop();
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => return true,
                    KeyCode::Char('v') => self.repoview.select_current(),
                    KeyCode::Char('V') => self.repoview.start_select_range(),
                    KeyCode::Char(':') | KeyCode::Char('/') => self.input.editing(true),
                    //KeyCode::Char('a') => self.apply_alias(),
                    KeyCode::Down => {
                        if self.repoview.select_mode {
                            self.repoview.select_current();
                        }
                        self.repoview.next();
                    }
                    KeyCode::Up => {
                        if self.repoview.select_mode {
                            self.repoview.select_current();
                        }
                        self.repoview.previous();
                    }
                    KeyCode::Left | KeyCode::Right => self.repoview.toggle_expanded(),
                    _ => {}
                }
            }
        }
        false
    }

    fn parse_command(&mut self) {
        // Command format: `:<command> <args>`
        let input = self.input.text();
        let cmd_str: Vec<&str> = input.split(" ").collect();
        if cmd_str.len() < 1 {
            self.input = Input::warning(String::from("No command"));
            return;
        }
        match cmd_str[0] {
            ":/" => unimplemented!(), // Search
            ":t" => self.repoview.tag_command(&cmd_str[1..]),
            ":a" => {
                if let Err(e) = self.repoview.alias_command(&cmd_str[1..]) {
                    self.input = e;
                }
            }
            _ => self.input.editing(false),
        }
    }
}
