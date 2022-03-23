use tui::{
    style::{Color, Modifier, Style},
    text::{Text, Span, Spans}
};
use crate::repo::{Repo, Status};
const COLLAPSED: &str = "▶ ";
const EXPANDED: &str = "▼ ";


pub struct RepoView<'a> {
    repo: &'a Repo,
    longest_name: usize,
    indent: u8,
    expanded: bool,
    selected: bool,
    status_start: u8,
}

impl<'a> RepoView<'a> {
    pub fn new(
        repo: &'a Repo, 
        longest_name: usize,
        indent: u8,
        expanded: bool,
        selected: bool,
        status_start: u8,
    ) -> Self {
        Self {
            repo,
            longest_name,
            indent,
            expanded,
            selected,
            status_start
        }
    }

    pub fn text(self) -> Vec<Spans<'a>> {
        let prefix = if self.expanded { String::from(EXPANDED) } else {String::from(COLLAPSED) };
        let name = self.repo.name.clone();
        let num_spaces =  (self.longest_name - name.len()) + 3;
        let spaces = (0..num_spaces).map(|_| " ").collect::<String>();
        let status = self.repo.status.as_ref().unwrap_or(&Status::Other);
        let mut spans = Vec::<Spans>::new();
        let first_line = Spans::from(vec![
            Span::raw(prefix),
            Span::raw(name),
            Span::raw(spaces),
            Span::styled(status.display(), Style::default().fg(get_color_for_status(status)))
        ]);
        spans.push(first_line);
        if self.expanded {
            spans.push(Spans::from(format!("    Branch: {}\r\n", self.repo.branch)));
            spans.push(Spans::from(format!("    Remotes: {:?}\r\n", self.repo.remotes)));
            spans.push(Spans::from(format!("    Alias: {:?}\r\n", self.repo.alias)));
            spans.push(Spans::from(format!("    Tags: {:?}\r\n", self.repo.tags)));
        }
        spans
    }

}

pub fn get_color_for_status(status: &Status) -> Color {
    match status {
        Status::Bare => Color::White,
        // Todo
        Status::Diverged(_,_) => Color::Red,
        Status::Clean => Color::Green,
        Status::Dirty => Color::Yellow,
        Status::Detached => Color::Red,
        Status::Other => Color::White
    }
}
