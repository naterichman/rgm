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


#[derive(Debug, PartialEq)]
pub struct RepoView {
    repo: Repo,
    tree: bool
    focused: bool,
    selected: bool,
}

impl Into<Text<'a>> for RepoView {

    pub fn get_content(&self) -> Text{
        // Main function to print a repo
        unimplemented!();
    }
}
