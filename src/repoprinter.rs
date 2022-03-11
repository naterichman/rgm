pub struct RepoPrinter {
    // Regular attributes
    // Line width, whether repo is focused, selected, expanded.
    // Terminal Width
    pub width: usize,
    pub focused: bool,
    pub selected: bool,
    pub expanded: bool

    // Tree view attributes: 
    // Indentation level, 
    pub indentation: usize,

}

const FOCUSED: &str = "▶ ";
const FOCUSED_COLLAPSED: &str = "▶ ";
const FOCUSED_EXPANDED: &str = "▼ ";
const COLLAPSED: &str = "▷ ";
const EXPANDED: &str = "▽ ";

const TREE_ITEM: &str = "├── ";
const TREE_INDENT: &str = "│   ";
const TREE_ITEM_LAST: &str = "└── ";

impl RepoPrinter {
    pub fn print(&self) {
    }
}
