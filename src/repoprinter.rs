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
pub trait Printer<T> {
    fn print(&self, writer: &mut T) -> Result<()>
        where T: Write;
}

pub struct FlatPrinter<'a>
{
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

impl<'a> FlatPrinter<'a> 
{
    pub fn new(width: usize, repo: &'a Repo, longest_name: usize) -> Self {
        Self {
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
    fn print_main_line<W>(&self, writer: &mut W) -> Result<()>
    where W: Write
    { 
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
        queue!(writer,
            SetBackgroundColor(bg),
            PrintStyledContent(prefix.with(fg)),
            SetBackgroundColor(bg),
            PrintStyledContent(name.with(fg).attribute(Attribute::Underlined)),
            MoveRight(num_spaces as u16),
            PrintStyledContent(status.with(s_color)),
            Print("\n\r")
        )?;
        Ok(())
    }
}


impl<T> Printer<T> for FlatPrinter<'_>
where
    T: Write,
{
    fn print(&self, writer: &mut T) -> Result<()> 
    where T: Write
    {
        if self.expanded {
            unimplemented!();
        } else {
            self.print_main_line(writer);
        }
        Ok(())
    }
}
