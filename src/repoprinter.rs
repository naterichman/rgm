use crossterm::{
    style::{Color, Attribute, Stylize, PrintStyledContent, Print, SetBackgroundColor},
    cursor::{MoveRight},
    Result,
    queue, QueueableCommand
};
use crate::repo::{Repo, Status};
use std::io::Write;

const COLLAPSED: &str = "▶ ";
const EXPANDED: &str = "▼ ";

const TREE_ITEM: &str = "├── ";
const TREE_INDENT: &str = "│   ";
const TREE_ITEM_LAST: &str = "└── ";

// Printer trait to define print method.
// Takes self, and an object that implements QueuableCommand, such as io::stdout() or
// io::stderr().
pub trait Printer {
    fn print(&mut self) -> Result<()>;
}

pub struct FlatPrinter<'a, 'b, T>
where T: QueueableCommand + Write,
{
    // Stdout, stderr, etc.
    writer: &'b mut T,
    // Terminal Width
    width: usize,
    // Height in lines of repo
    height: usize,
    // Possible modifications from actions
    focused: bool,
    selected: bool,
    expanded: bool,
    // Actual repo
    repo: &'a Repo,

    longest_name: usize,
    
}

impl<'a, 'b, T> FlatPrinter<'a, 'b, T> 
where
    T: QueueableCommand + Write
{
    pub fn new(writer: &'b mut T, width: usize, repo: &'a Repo, longest_name: usize) -> Self {
        Self {
            writer,
            width,
            height: 1,
            focused: false,
            selected: false,
            expanded: false,
            repo,
            longest_name
        }
    } 

    pub fn height(&self) -> usize { self.height }

    pub fn toggle_selected(&mut self) { self.selected = !self.selected }

    pub fn toggle_expanded(&mut self) { self.expanded = !self.expanded }

    pub fn toggle_focused(&mut self) { self.focused = !self.focused }

    // Four parts to the first line, [prefix][name][tabs][status]
    fn print_main_line(&mut self) -> Result<()>{ 
        let prefix = if self.expanded { String::from(EXPANDED) } else {String::from(COLLAPSED) };
        let name = self.repo.name.clone();
        let num_spaces = 5 + (self.longest_name - name.len());
        let (status, s_color) = self.repo.status.as_ref().unwrap_or(&Status::Other).display();
        // Set line background color based on whether line is selected and/or focused
        let (fg, bg) = match (self.focused, self.selected) {
            // Both selected and focused
            (true, true) => (Color::Black, Color::DarkRed),
            // Focused, but not selected
            (true, false) => (Color::Black, Color::DarkGreen), 
            // Selected, but not focused
            (false, true) => (Color::Black, Color::DarkGreen),
            // Default
            (false, false) => (Color::White, Color::Reset)
        };
        queue!(self.writer,
            SetBackgroundColor(bg),
            PrintStyledContent(prefix.with(fg)),
            PrintStyledContent(name.with(fg).attribute(Attribute::Underlined)),
            MoveRight(num_spaces as u16),
            PrintStyledContent(status.with(s_color)),
            Print("\n\r")
        )?;
        Ok(())
    }
}


impl<T> Printer for FlatPrinter<'_, '_, T>
where
    T: QueueableCommand + Write,
{
    fn print(&mut self) -> Result<()> {
        if self.expanded {
            unimplemented!();
        } else {
            self.print_main_line();
        }
        Ok(())
    }
}
