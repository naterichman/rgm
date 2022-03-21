use args::{Cli, Commands};
use clap::Parser;
use repo::Repos;
use std::{
    process,
    io,
    error::Error,
    time::{Duration, Instant}
};
use logging::setup_log;
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
use repoview::RepoView;

use crate::repo::Repo;

mod args;
mod repo;
mod error;
mod logging;
mod repoview;

fn usage(){
    println!("rgm PATH")
}

fn get_repos_or_exit() -> Repos {
    let repos = Repos::load();
    match repos {
        Ok(r) => r,
        Err(_) => process::exit(1)
    }
}

fn main() {
    setup_log().unwrap();
    log::info!("Set up logging");
    let cli = Cli::parse();
    match cli.command {
        Some(command) => match command {
            Commands::Tag{mut tags, path} => {
                let mut repos = get_repos_or_exit();
                for r in repos.repos.iter_mut(){
                    // if r.path is a subdirectory of path
                    if r.path.starts_with(&path){
                        r.add_tags(&mut tags);
                    }
                }
            },
            Commands::Alias{alias, path} => {
                let mut repos = get_repos_or_exit();
                // Match on path
                for r in repos.repos.iter_mut() {
                    if r.path == path {
                        println!("Adding alias {} to {}",&alias,&r.name);
                        r.add_alias(alias);
                        break
                    }
                }
            },
            Commands::Import{path} => {
                let repos = Repos::from(&path);
                match repos.save() {
                    Ok(p) => println!("Saved {} repos to {}", &repos.meta.size, p.display()),
                    Err(e) => println!("Error saving repos: {}", e)
                }
            },
        },
        None => {
            let repos = Repos::load();
            match repos {
                Ok(r) => if let Some(e) = run_interactive(r).err() {
                    println!("{:?}",e)
                },
                Err(e) => println!("{:?}", e)
            }
            //match repos {
            //   Ok(r) => {
            //        for val in r.repos.iter() {
            //            println!("{:?}", val)
            //        }
            //    },
            //    Err(e) => println!("{:?}", e)
           // }
        }
    }
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
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
}

struct App {
    items: StatefulList<Repo>,
    expanded: Vec<usize>
}

impl App {
    pub fn new(repos: Repos) -> Self {
        Self {
            items: StatefulList::with_items(repos.repos),
            expanded: Vec::<usize>::new(),
        }
    }
}

pub fn run_interactive(repos: Repos) -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let mut app = App::new(repos);
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let items: Vec<ListItem> = app
                .items
                .items
                .iter()
                .enumerate()
                .map(|(i, repo)| {
                    // TODO: RepoView::get_text
                    let repo_view = RepoView::new(
                        &repo,
                        0,
                        app.expanded.contains(&i),
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
            f.render_stateful_widget(items, size, &mut app.items.state);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
