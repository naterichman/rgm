use std::{
    io,
    error::Error,
    time::{Duration, Instant}
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Direction, Layout, Constraint},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{info, debug};
use crate::repoview::RepoView;
use crate::repo::{Repos, Repo};
use crate::utils;

pub struct Input {
    editing: bool,
    text: String,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            editing: false,
            text: String::from(":")
        }
    }
}


pub struct Screen {
    items: StatefulList<Repo>,
    expanded: Vec<usize>,
    longest_name: usize,
    select_mode: bool,
    selected: Vec<usize>,
    input: Input
}


impl Screen {
    pub fn new(repos: Repos) -> Self {
        let longest = repos.longest_name();
        Self {
            items: StatefulList::new(repos.repos),
            expanded: Vec::<usize>::new(),
            longest_name: longest,
            select_mode: false,
            selected: Vec::<usize>::new(),
            input: Input::default()
        }
    }
    fn get_terminal<W>(mut writer: W) -> Result<Terminal<CrosstermBackend<W>>, Box<dyn Error>> 
    where W: io::Write
    {
        enable_raw_mode()?;
        execute!(writer, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(writer);
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }
 
    pub fn run<W>(&mut self, writer: W) -> Result<(), Box<dyn Error>>
    where W: io::Write
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
                if should_exit { break; } 
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
                .constraints(
                    [
                        Constraint::Length(&size.height - 1),
                        Constraint::Length(1)
                    ].as_ref())
                .split(size);
            let items: Vec<ListItem> = self
                .items
                .items
                .iter()
                .enumerate()
                .map(|(i, repo)| {
                    // TODO: Add selected as a param here, change color of Status for better
                    // rendering on background
                    let repo_view = RepoView::new(
                        &repo,
                        self.longest_name,
                        0,
                        self.expanded.contains(&i),
                        false,
                        30u8
                    );
                    let selected = self.selected.contains(&i);
                    let (b_color, f_color) = if selected { 
                        (Color::Rgb(40,40,40), Color::White)
                    } else {
                        (Color::Reset, Color::White)
                    };
                    ListItem::new(repo_view.text())
                        .style(Style::default().bg(b_color).fg(f_color))
                })
                .collect();

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Repositories"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightBlue)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
            // Render list of repos
            f.render_stateful_widget(items, chunks[0], &mut self.items.state);
            // Render input
            let input = Paragraph::new(self.input.text.as_ref())
                .block(Block::default());
            f.render_widget(input, chunks[1]);
        });
        match res {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn exit<B>(&self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>>
    where B: Backend
    {
        disable_raw_mode()?;
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn handle_event(&mut self) -> bool {
        let raw_evt = match event::read() {
            Ok(evt) => evt,
            Err(e) => {
                println!("{}", e.to_string());
                return true
            }
        };

        if let Event::Key(key) = raw_evt {
            if self.input.editing {
                match key.code {
                    KeyCode::Char(x) => self.input.text.push(x),
                    KeyCode::Enter => self.parse_command(),
                    KeyCode::Backspace => { self.input.text.pop(); },
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => return true,
                    KeyCode::Char('v') => self.select_current(),
                    KeyCode::Char('V') => self.start_select_range(),
                    KeyCode::Char(':') | KeyCode::Char('/') => self.input.editing = true,
                    //KeyCode::Char('a') => self.apply_alias(),
                    KeyCode::Down => {
                        if self.select_mode {
                            self.select_current();
                        }
                        self.items.next();
                    },
                    KeyCode::Up => {
                        if self.select_mode {
                            self.select_current();
                        }
                        self.items.previous();
                    },
                    KeyCode::Left | KeyCode::Right => self.toggle_expanded(),
                    _ => {}
                }
            }
        }
        false
    }

    fn parse_command(&mut self) {
        self.input.text.remove(0);
        let command_char = self.input.text.remove(0);
        match command_char {
            '/' => unimplemented!(), // Search
            't' => unimplemented!(), // Tag
            'a' => unimplemented!(), // Alias
            _ => {}
        }
    }

    fn select_current(&mut self) {
        if let Some(s) = self.items.selected(){
            debug!("Selecting {}", s);
            utils::set_item_in_vec(&mut self.selected, s);
        }
    }

    fn start_select_range(&mut self) {
        self.select_mode = true;
        debug!("Starting select range");
        self.select_current()
    }

    fn toggle_expanded(&mut self){
        if let Some(s) = self.items.selected(){
            utils::toggle_item_in_vec(&mut self.expanded, s);
        }
    }
}

// Basic stateful list from example on tui-rs
struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn new(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn selected(&self) -> Option<usize> { self.state.selected() }
}
