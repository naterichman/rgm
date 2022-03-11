use crossterm::{
    event::{Event, KeyCode, read, poll},
    tty::IsTty,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen,
        enable_raw_mode, disable_raw_mode, size
    },
    execute,
    queue, QueueableCommand, cursor,
    style::{Print,Stylize}
};
use std::ops::{Range, Drop};
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use crate::repo::{Repos, Repo, Status};
use crate::error::{Result, RgmError};

#[derive(Debug, PartialEq)]
pub enum Action{
    // Deselect index/indices
    Deselect(Range<usize>),
    // Select index/indices
    Select(Range<usize>),
    // Enter an aliaas for selected repo
    Alias,
    // Enter a tag for selected repo(s)
    Tag,
    // Input Action for tag and alias
    Input,
    // Move up or down
    MoveDown(usize),
    MoveUp(usize),
    // Toggle tree view
    ToggleTree,
    // Toggle expanded or collapsed
    ToggleCollapsed,
    // Quit
    Exit,
    // No action
    Nil
}

impl Action {
    pub fn needs_update(&self) -> bool {
        match self {
            Self::Deselect(_) => true,
            Self::Select(_) => true,
            Self::MoveUp(_) => true,
            Self::MoveDown(_) => true,
            Self::Alias => true,
            Self::Tag => true,
            Self::Input => false,
            Self::ToggleTree => true,
            Self::ToggleCollapsed => true,
            Self::Exit => false,
            Self::Nil => false,
        }
    }
}

pub struct Screen {
    repos: Repos,
    height: usize,
    width: usize,
    tree_view: bool
}

impl Screen {
    pub fn new(repos: Repos) -> Self {
        if !stdin().is_tty() { 
            // Run without screen
            unimplemented!()
        } else {
            let (height, width) = size().unwrap();
            Self {
                repos,
                height: height as usize,
                width: width as usize,
                tree_view: false
            }
        }
    }
    pub fn start(&self) -> Result<()> {
        self.write_repos();
        execute!(stdout(), EnterAlternateScreen)
            .map_err(|err| RgmError{ message: err.to_string() })?;
        enable_raw_mode()
            .map_err(|err| RgmError{ message: err.to_string() })?;
        Ok(())
    }

    fn write_repo(&self, repo: &Repo) {
        if self.tree_view {
            unimplemented!()
        } else {
            let status = repo.status.as_ref().unwrap_or(&Status::Other);
            stdout().queue(Print(format!( "{} {:?}\r\n", repo.name, status).red()));
        }
    }
    
    fn write_repos(&self) {
        // TODO: Write repos in tree mode
        for repo in self.repos.repos.iter()  {
            self.write_repo(&repo)
        }
        stdout().flush();
    }
    
    fn get_event(&self) -> Option<Event> {
        if poll(Duration::from_millis(500)).unwrap() {
            Some(read().unwrap())
        } else {
            None
        }
    }

    fn handle_event(&mut self, event: Option<&Event>) -> Action {
        match event {
            Some(evt) => {
                if *evt == Event::Key(KeyCode::Char('q').into()) {
                    Action::Exit
                } else {
                    Action::Nil
                }
            },
            None => Action::Nil
        }
    }

    // Simple update until q is pressed.
    pub fn update(&mut self) -> bool{
        let event = self.get_event();
        let action = self.handle_event(event.as_ref());
        if action.needs_update() {
            self.write_repos()
        }
        if action == Action::Exit {
            false
        } else {
            true
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
