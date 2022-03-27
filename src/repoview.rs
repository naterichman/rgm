use crate::input::Input;
use crate::repo::{Repo, Repos};
use crate::repoitem::RepoItem;
use crate::screen::Draw;
use crate::utils;
use log::{debug, info};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    terminal::Frame,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub struct RepoView {
    items: StatefulList<Repo>,
    expanded: Vec<usize>,
    column_widths: Vec<usize>,
    pub select_mode: bool,
    selected: Vec<usize>,
}

impl RepoView {
    pub fn new(repos: Repos) -> Self {
        let longest = repos.longest_name();
        Self {
            items: StatefulList::new(repos.repos),
            expanded: Vec::<usize>::new(),
            column_widths: vec![longest],
            select_mode: false,
            selected: Vec::<usize>::new(),
        }
    }

    pub fn tag_command(&mut self, cmd: &[&str]) {
        // Convert to Vec<String>
        let mut tags = cmd.iter().map(|v| String::from(*v)).collect();
        for idx in &self.selected[..] {
            self.items.items[*idx].add_tags(&mut tags);
        }
    }

    pub fn alias_command(&mut self, cmd: &[&str]) -> Result<(), Input> {
        let alias = cmd.join("-");
        if self.selected.len() > 1 {
            return Err(Input::warning(String::from(
                "Not applying alias to multiple selected repos",
            )));
        } else {
            self.items.items[self.selected[0]].add_alias(alias);
        }
        Ok(())
    }

    pub fn select_current(&mut self) {
        if let Some(s) = self.items.selected() {
            debug!("Selecting {}", s);
            utils::set_item_in_vec(&mut self.selected, s);
        }
    }

    pub fn start_select_range(&mut self) {
        self.select_mode = true;
        debug!("Starting select range");
        self.select_current()
    }

    pub fn toggle_expanded(&mut self) {
        if let Some(s) = self.items.selected() {
            utils::toggle_item_in_vec(&mut self.expanded, s);
        }
    }

    pub fn next(&mut self) {
        self.items.next();
    }

    pub fn previous(&mut self) {
        self.items.previous();
    }
}

impl Draw for RepoView {
    fn draw<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self
            .items
            .items
            .iter()
            .enumerate()
            .map(|(i, repo)| {
                // TODO: Add selected as a param here, change color of Status for better
                // rendering on background
                let repo_view = RepoItem::new(
                    &repo,
                    self.column_widths[0],
                    0,
                    self.expanded.contains(&i),
                    false,
                    30u8,
                );
                let selected = self.selected.contains(&i);
                let (b_color, f_color) = if selected {
                    (Color::Rgb(40, 40, 40), Color::White)
                } else {
                    (Color::Reset, Color::White)
                };
                ListItem::new(repo_view.text()).style(Style::default().bg(b_color).fg(f_color))
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Repositories"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_stateful_widget(items, area, &mut self.items.state);
    }
}
// Basic stateful list from example on tui-rs
struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn new(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
}
