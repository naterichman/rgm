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
use std::ops::Drop;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use crate::repo::Repos;
use crate::error::{Result, RgmError};

#[derive(Debug, PartialEq)]
pub enum Action{
    Exit,
    Nil
}

impl Action {
    pub fn needs_update(&self) -> bool {
        match self {
            Self::Exit => false,
            Self::Nil => false,
        }
    }
}

pub struct Screen {
    repos: Repos,
    height: usize,
    width: usize,
}

impl Screen {
    pub fn new(repos: Repos) -> Result<Self> {
        if !stdin().is_tty() { 
            Err(RgmError { 
                message: String::from("Cannot run RGM, stdin is not a tty")
            })
        } else {
            let (height, width) = size()
                .map_err(|err| RgmError{ message: err.to_string() })?;
            execute!(stdout(), EnterAlternateScreen)
                .map_err(|err| RgmError{ message: err.to_string() })?;
            enable_raw_mode()
                .map_err(|err| RgmError{ message: err.to_string() })?;
            Ok(Self {
                repos,
                height: height as usize,
                width: width as usize,
            })
        }
    }
    pub fn start(&self) {
        self.write_repos();
    }
    
    fn write_repos(&self) {
        for repo in self.repos.repos.iter()  {
            stdout().queue(
                Print(
                    format!( "{} {:?}", repo.path.display(), repo.status).red()
                ),
            );
            stdout().queue(cursor::MoveDown(1));
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
        // Destroy events iterator?
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
