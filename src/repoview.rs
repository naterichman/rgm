use tui::{
    style::{Color, Modifier, Style},
    text::{Text, Span, Spans}
};
use crate::repo::{Repo, Status};
const COLLAPSED: &str = "▶ ";
const EXPANDED: &str = "▼ ";


pub struct RepoView<'a> {
    repo: &'a Repo,
    indent: u8,
    expanded: bool,
    selected: bool,
    status_start: u8,
}

impl<'a> RepoView<'a> {
    pub fn new(
        repo: &'a Repo, 
        indent: u8,
        expanded: bool,
        selected: bool,
        status_start: u8,
    ) -> Self {
        Self {
            repo,
            indent,
            expanded,
            selected,
            status_start
        }
    }

    pub fn text(self) -> Vec<Spans<'a>> {
        let prefix = if self.expanded { String::from(EXPANDED) } else {String::from(COLLAPSED) };
        let name = self.repo.name.clone();
        let num_spaces =  10u8;//self.status_start - name.len() as u8;
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
