// Based on pulldown_cmark's HtmlWriter.

use std::cell::RefCell;
use std::rc::Rc;

use crate::routines::markdown::types as public_types;

#[derive(Clone, Default, Debug)]
pub struct Root {
    pub nodes: Vec<Node>,
}

// <pre><code>...</code></pre>
#[derive(Clone, Default, Debug)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// <p>...</p>
#[derive(Clone, Default, Debug)]
pub struct Paragraph {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// <h1 - h6>...</h1 - h6>
#[derive(Clone, Default, Debug)]
pub struct Heading {
    pub level: u32,
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// <blockquote>...</blockquote>
#[derive(Clone, Default, Debug)]
pub struct Blockquote {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// no start: ul, Unordered List
// start: ol, Ordered List
#[derive(Clone, Default, Debug)]
pub struct List {
    pub start: Option<u64>,
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// <li>...</li>
#[derive(Clone, Default, Debug)]
pub struct ListItem {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// <em>...</em>
#[derive(Clone, Default, Debug)]
pub struct Emphasis {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// <strong>...</strong>
#[derive(Clone, Default, Debug)]
pub struct Strong {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// <del>...</del>
#[derive(Clone, Default, Debug)]
pub struct Strikethrough {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

// <img src="..." title="..." />
#[derive(Clone, Default, Debug)]
pub struct Image {
    pub src: String,
    pub alt: String,
    pub title: Option<String>,
}

// <img src="..." title="..." />
#[derive(Clone, Default, Debug)]
pub struct HyperLink {
    pub href: String,
    pub title: Option<String>,

    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

//#[derive(Debug)]
// enum TableState {
// Head,
// Body,
//}

// <table>...</table>
// struct Table {
// children: Vec<Node>,
// parent: Rc<RefCell<Option<Node>>>,

// table_state: TableState,
// table_alignments: Vec<Alignment>,
// table_cell_index: usize,
//}

// impl Default for Table {
// fn default() -> Self {
// Self {
// children: Vec::new(),
// parent: Rc::new(RefCell::new(None)),

// table_state: TableState::Head,
// table_alignments: Vec::new(),
// table_cell_index: 0,
//}

// impl fmt::Debug for Table {
// fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// f.debug_struct("Table")
//.field("children", &self.children)
//.field("parent", &self.parent)
//.field("table_state", &"TableState")
//.field("table_alignments", &self.table_alignments)
//.field("table_cell_index", &self.table_cell_index)
//.finish()
//}

// <input type="checkbox" disabled>
#[derive(Clone, Default, Debug)]
pub struct Checkbox {
    pub checked: bool,
}

impl From<Checkbox> for public_types::Checkbox {
    fn from(value: Checkbox) -> Self {
        Self {
            checked: value.checked,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    Text(String),
    Code(String),
    Html(String),
    Paragraph(Paragraph),
    Heading(Heading),
    // Table(Table),

    // \n
    SoftBreak,

    // <br />
    HardBreak,

    // <hr />
    Rule,

    Checkbox(Checkbox),

    Blockquote(Blockquote),
    CodeBlock(CodeBlock),
    List(List),
    ListItem(ListItem),

    Emphasis(Emphasis),
    Strong(Strong),
    Strikethrough(Strikethrough),

    Image(Image),

    HyperLink(HyperLink),
}
