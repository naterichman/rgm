use crossterm::{
    style::{Color, Attribute, Stylize, PrintStyledContent, Print, SetBackgroundColor},
    cursor::{MoveRight},
    terminal::{Clear, ClearType},
    Result,
    queue, QueueableCommand
};
use crate::repo::{Repo, Status};
use std::io::Write;
use log::info;

const COLLAPSED: &str = "▶ ";
const EXPANDED: &str = "▼ ";

const TREE_ITEM: &str = "├── ";
const TREE_INDENT: &str = "│   ";
const TREE_ITEM_LAST: &str = "└── ";

// Printer trait to define print method.
// Takes self, and an object that implements QueuableCommand, such as io::stdout() or
// io::stderr().
pub trait RepoPrinter<T> {
    // TODO: I don't like passing in longest name here, but
    /// Print
    fn print(&mut self, writer: &mut T, longest_name: usize) -> Result<()>
        where T: Write;

    /// Get current height of printer
    fn height(&self) -> usize;

    /// Toggle whether printer is selected
    fn toggle_selected(&mut self);

    /// Toggle whether printer is expanded or collapsed
    fn toggle_expanded(&mut self);

    /// Toggle focused
    fn toggle_focused(&mut self);

    /// Accessor for repo
    fn get_repo(&self) -> &Repo;
}

pub struct FlatPrinter
{
    repo: Repo,
    // Terminal Width
    width: usize,
    // Height in lines of repo
    height: usize,
    // Possible modifications from actions
    focused: bool,
    selected: bool,
    expanded: bool,
    
}

impl FlatPrinter
{
    pub fn new(repo: Repo, width: usize) -> Self {
        Self {
            repo,
            width,
            height: 1,
            focused: false,
            selected: false,
            expanded: false,
        }
    } 

    // Four parts to the first line, [prefix][name][tabs][status]
    fn print_main_line<W>(&self, writer: &mut W, longest_name: usize) -> Result<usize>
    where W: Write
    { 
        let prefix = if self.expanded { String::from(EXPANDED) } else {String::from(COLLAPSED) };
        let name = self.repo.name.clone();
        let num_spaces = 5 + (longest_name - name.len());
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
        Ok(1)
    }

    fn print_details<W>(&self, writer: &mut W) -> Result<usize>
    where W: Write
    {
        info!("Writing details");
        let branch = format!("\tBranch: {}\r\n", self.repo.branch);
        let remotes = format!("\tRemotes: {:?}\r\n", self.repo.remotes);
        let alias = format!("\tAlias: {:?}\r\n", self.repo.alias);
        let tags = format!("\tTags: {:?}\r\n", self.repo.tags);

        queue!(writer,
               Print(branch),
               Print(remotes),
               Print(alias),
               Print(tags),
        )?;
        Ok(4)

    }
}


impl<T> RepoPrinter<T> for FlatPrinter
where
    T: Write,
{
    fn height(&self) -> usize { self.height }

    fn toggle_selected(&mut self) { self.selected = !self.selected }

    fn toggle_expanded(&mut self) { self.expanded = !self.expanded }

    fn toggle_focused(&mut self) { self.focused = !self.focused }

    fn get_repo(&self) -> &Repo { &self.repo }

    fn print(&mut self, writer: &mut T, longest_name: usize) -> Result<()> 
    where T: Write
    {
        self.height = self.print_main_line(writer, longest_name).unwrap();
        if self.expanded { self.height += self.print_details(writer).unwrap(); }
        Ok(())
    }
}
