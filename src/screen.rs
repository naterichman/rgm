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
use crate::repoprinter::{FlatPrinter, Printer};

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
    repos: Repos,
    writer: T,
    height: usize,
    width: usize,
    focused: usize,
    //
    longest_name: usize,
    tree_view: bool,
    selected: Vec<usize>,
    start_idx: usize,
}

impl<T> Screen<T>
where T: Write
{
    pub fn new(repos: Repos, writer: T) -> Self {
        if !stdin().is_tty() { 
            // Run without screen
            unimplemented!()
        } else {
            let (width, height) = size().unwrap();
            let mut longest_name = 0;
            for repo in repos.repos.iter() {
                if repo.name.len() > longest_name {
                    longest_name = repo.name.len();
                }
            }
            Self {
                repos,
                height: height as usize,
                width: width as usize,
                focused: 0,
                longest_name,
                tree_view: false,
                selected: Vec::<usize>::new(),
                start_idx: 0,
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
        self.clear();
        let mut lines = 0;
        let mut idx = 0;
        // TODO: Write repos in tree mode
        while lines < self.height - 1 {
            let repo = &self.repos.repos[idx];
            let mut printer = FlatPrinter::new(
                self.width,
                repo,
                self.longest_name
            );
            if idx == self.focused { printer.toggle_focused(); }
            if self.selected.contains(&idx) { printer.toggle_selected(); }
            printer.print(&mut self.writer);
            lines += printer.height();
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
                if self.focused != 0 {
                    self.focused = self.focused - 1;
                }
                Action::Move
            },
            KeyCode::Down => {
                if self.focused != self.repos.repos.len() {
                    self.focused = self.focused + 1;
                }
                Action::Move
            },
            _ => Action::Nil
        }
    }

    // Simple update until q is pressed.
    pub fn update(&mut self) -> bool{
        let event = self.get_event();
        let action = self.handle_event(event);
        if action.needs_update() {
            self.write_repos();
        }
        if action == Action::Exit {
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
