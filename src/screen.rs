use std::{
    io,
    error::Error,
    time::{Duration, Instant}
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::repoview::RepoView;
use crate::repo::{Repos, Repo};

pub struct Screen {
    items: StatefulList<Repo>,
    expanded: Vec<usize>,
    longest_name: usize,
}


impl Screen {
    pub fn new(repos: Repos) -> Self {
        let longest = repos.longest_name();
        Self {
            items: StatefulList::new(repos.repos),
            expanded: Vec::<usize>::new(),
            longest_name: longest
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
            let items: Vec<ListItem> = self
                .items
                .items
                .iter()
                .enumerate()
                .map(|(i, repo)| {
                    let repo_view = RepoView::new(
                        &repo,
                        self.longest_name,
                        0,
                        self.expanded.contains(&i),
                        false,
                        30u8
                    );
                    ListItem::new(repo_view.text())
                        .style(Style::default().bg(Color::Reset))
                })
                .collect();

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Repositories"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightBlue)
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );

            // We can now render the item list
            f.render_stateful_widget(items, size, &mut self.items.state);
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
            match key.code {
                KeyCode::Char('q') => return true,
                KeyCode::Down => self.items.next(),
                KeyCode::Up => self.items.previous(),
                KeyCode::Left | KeyCode::Right => if let Some(i) = self.items.selected() {
                    if self.expanded.contains(&i) {
                        self.expanded.retain(|&x| x != i);
                    } else {
                        self.expanded.push(i);
                    }
                },
                _ => {}
            }
        }
        false
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
