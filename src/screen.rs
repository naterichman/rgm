use crossterm::{
    event::{Event, KeyCode, read, poll, KeyEvent},
    tty::IsTty,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen,
        enable_raw_mode, disable_raw_mode, size, Clear, ClearType
    },
    execute,
    queue, QueueableCommand, cursor, ExecutableCommand,
    style::{Print,Stylize}
};
use std::ops::{Range, Drop};
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use crate::repo::{Repos, Repo, Status};
use crate::error::{Result, RgmError};
use crate::repoprinter::{FlatPrinter, RepoPrinter};
use log::{info};

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
    Move,
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
            Self::Move => true,
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

pub struct Screen<T>
where T: Write
{
    repos: Vec<Box<dyn RepoPrinter<T>>>,
    writer: T,
    height: usize,
    width: usize,
    //
    longest_name: usize,
    tree_view: bool,
    /// Index in self.repos where we start printing to screen
    screen_start: usize,
    /// Index in self.repos that is currently focused
    focused: usize,
    /// Indices that are currently selected
    selected: Vec<usize>,
}

impl<T> Screen<T>
where T: Write
{
    pub fn new(raw_repos: Repos, writer: T) -> Self {
        if !stdin().is_tty() { 
            // Run without screen
            unimplemented!()
        } else {
            let (width, height) = size().unwrap();
            let mut longest_name = 0;
            let mut repos = Vec::<Box<dyn RepoPrinter<T>>>::new();
            for repo in raw_repos.repos.into_iter() {
                if repo.name.len() > longest_name { longest_name = repo.name.len(); }
                repos.push(Box::new(FlatPrinter::new(repo, width as usize)));
            }
            repos[0].toggle_focused();
            Self {
                repos,
                height: height as usize,
                width: width as usize,
                focused: 0,
                longest_name,
                tree_view: false,
                selected: Vec::<usize>::new(),
                screen_start: 0,
                writer
            }
        }
    }
    pub fn start(&mut self) -> Result<()> {
        // Enter alternate screen and enter raw mode.
        self.writer.execute(EnterAlternateScreen)
            .map_err(|err| RgmError{ message: err.to_string() })?;
        enable_raw_mode()
            .map_err(|err| RgmError{ message: err.to_string() })?;
        self.write_repos();
        Ok(())
    }
    
    fn clear(&mut self) -> Result<()> {
        self.writer.execute(cursor::MoveTo(0,0))
            .map_err(|err| RgmError{ message: err.to_string() })?;
        self.writer.execute(Clear(ClearType::All))
            .map_err(|err| RgmError{ message: err.to_string() })?;
        Ok(())
    }
    
    fn write_repos(&mut self) -> Result<()> {
        // TODO: Clear less if possible
        self.clear();
        let mut lines = 0;
        let mut idx = self.screen_start;
        // TODO: Write repos in tree mode
        while lines < self.height - 1 {
            let repo = &mut self.repos[idx];
            repo.print(&mut self.writer, self.longest_name);
            lines += repo.height();
            idx += 1;
        }
        self.writer.flush();
        Ok(())
    }
    
    fn get_event(&self) -> Option<Event> {
        if poll(Duration::from_millis(500)).unwrap() {
            Some(read().unwrap())
        } else {
            None
        }
    }

    fn handle_event(&mut self, event: Option<Event>) -> Action {
        if let Some(evt) = event {
            match evt {
                Event::Key(key_evt) => self.handle_key_event(key_evt),
                _ => Action::Nil
            }
        } else {
            Action::Nil
        }
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> Action{
        match event.code {
            KeyCode::Char('q') => Action::Exit,
            KeyCode::Up => {
                // Only move up if we're not at the beginning of repos
                if self.focused != 0 {
                    if self.focused == self.screen_start {
                        // We're at the top of the screen currently, need to shift our window up
                        // one.
                        self.screen_start -= 1;
                    }
                    // Move focused index up
                    self.focused -= 1;
                    // Unfocus previous line
                    self.repos[self.focused + 1].toggle_focused();
                    // Focus current line
                    self.repos[self.focused].toggle_focused();
                }
                info!("focused: {}, screen_start: {}, height: {}",self.focused, self.screen_start, self.height);
                Action::Move
            },
            KeyCode::Down => {
                // Only move down if we're not at the end of repos
                if self.focused != self.repos.len() {
                    if (self.focused - self.screen_start) == self.height - 2 {
                        // We're at the bottom of the screen currently, need to shift our window
                        // down one.
                        self.screen_start += 1;
                    }
                    // Move focused index down
                    self.focused += 1;
                    // Unfocus previous line
                    self.repos[self.focused - 1].toggle_focused();
                    // Focus current line
                    self.repos[self.focused].toggle_focused();
                }
                info!("focused: {}, screen_start: {}, height: {}",self.focused, self.screen_start, self.height);
                Action::Move
            },
            KeyCode::Left => {
                self.repos[self.focused].toggle_expanded();
                info!("Toggling expansion: {}",self.focused);
                Action::ToggleCollapsed
            },
            KeyCode::Right => {
                self.repos[self.focused].toggle_expanded();
                info!("Toggling expansion: {}",self.focused);
                Action::ToggleCollapsed
            },
            _ => Action::Nil
        }
    }

    // Simple update until q is pressed.
    pub fn update(&mut self) -> bool{
        let event = self.get_event();
        let action = self.handle_event(event);
        if action.needs_update() {
            self.write_repos().unwrap();
        }
        if action == Action::Exit {
            info!("Exiting");
            false
        } else {
            true
        }
    }
}

impl<T> Drop for Screen<T>
where T: Write {
    fn drop(&mut self) {
        execute!(self.writer, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
