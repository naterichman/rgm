use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::Style,
    text::Text,
    widgets::{Block, StatefulWidget, Widget},
};

use crate::repo::Repos;

const COLLAPSED_FOCUSED: &str = "▶ ";
const COLLAPSED_UNFOCUSED: &str = "▷ ";
const EXPANDED_FOCUSED: &str = "▼ ";
const EXPANDED_UNFOCUSED: &str = "▽ ";

const TREE_ITEM: &str = "├── ";
const TREE_INDENT: &str = "│   ";
const TREE_ITEM_LAST: &str = "└── ";


// Based of ListItem in tui::widgets::List
// Single element of a line.
#[derive(Debug, Clone, PartialEq)]
pub struct LineElement<'a> {
    content: Text<'a>,
    style: Style
}

impl<'a> LineElement<'a> {
    pub fn new<T>(content: T) -> LineElement<'a>
    where
        T: Into<Text<'a>>,
    {
        LineElement {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> LineElement<'a> {
        self.style = style;
        self
    }

    pub fn width(&self) -> usize {
        self.content.width()
    }
}

// A line is made of multiple elements in order
pub struct Line{
    elements: Vec<LineElement>,
    focused: bool,
    selected: bool,
};

impl Line {
    // Focus change
    pub fn focus(&mut self){ self.focused = true; }
    pub fn unfocus(&mut self){ self.focused = false; }
    pub fn toggle_focus(&mut self){ self.focused = !self.focused; }

    // Select change
    pub fn select(&mut self){ self.selected = true; }
    pub fn unselect(&mut self){ self.selected = false; }
    pub fn toggle_select(&mut self){ self.selected = !self.selected; }

    pub fn new(elements: Vec<LineElement>) -> Self {
        Line {
            elements,
            focused: false,
            selected: false,
        }
    }
}

// A RepoView is made of one or more Lines
pub struct RepoView {
    lines: Vec<Line>,
    tree: bool,
}

impl RepoView {
    pub fn draw(&mut self, buf: &mut Buffer){
        unimplemented!();
    }
}



pub struct RepoViewerState {
    pub selected: Vec<usize>,
    pub focused: usize,
    pub offset: usize,
}

pub struct RepoViewer {
    pub repos: Vec<RepoView>,
    pub tree: bool,
    pub start_corner: Corner
}
impl RepoViewer {
    pub fn new(repos: Repos){
    }
    pub fn handle_event() {
        unimplemented!()
    }
}

impl StatefulWidget for RepoViewer {
    type State = RepoViewerState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let viewer_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        // Area too small
        if viewer_area.width < 1 || viewer_area.height < 1 { return; }
        // Nothing to render
        if self.repos.is_empty() { return; }

        let height = 0u16;
        let idx = 0u16;
        while height < viewer_area.height {
            let repo_view = self.repos[idx];
            let (x, y) = match self.start_corner {
                Corner::BottomLeft => {
                    height += repo_view.height() as u16;
                    (viewer_area.left(), viewer_area.bottom() - current_height)
                }
                _ => {
                    let pos = (viewer_area.left(), viewer_area.top() + current_height);
                    current_height += repo_view.height() as u16;
                    pos
                }
            };
            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: repo_view.height() as u16,
            };
            let repo_style = self.style.patch(repo_view.style);
            buf.set_style(area, repo_style);
            repo_view.draw(buf);
        }
    }
}
