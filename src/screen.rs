use crossterm::{
    event::{Event, KeyCode, read, poll},
    tty::IsTty,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen,
        enable_raw_mode, disable_raw_mode, size
    },
    execute
};
use std::ops::Drop;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use crate::repo::Repos;
use crate::error::{Result, RgmError};

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

    // Simple update until q is pressed.
    pub fn update(&mut self) -> bool{
        let event = {
            if poll(Duration::from_millis(500)).unwrap() {
                read().unwrap()
            } else {
                return true
            }
        };
        if event == Event::Key(KeyCode::Char('q').into()) {
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
