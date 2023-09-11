// Based on pulldown_cmark's HtmlWriter.

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document {
    pub children: Vec<Node>,
}

// <pre><code>...</code></pre>
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub children: Vec<Node>,
}

// <p>...</p>
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Paragraph {
    pub children: Vec<Node>,
}

// <h1 - h6>...</h1 - h6>
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Heading {
    pub level: u32,
    pub children: Vec<Node>,
}

// <blockquote>...</blockquote>
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Blockquote {
    pub children: Vec<Node>,
}

// no start: ul, Unordered List
// start: ol, Ordered List
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct List {
    pub start: Option<u64>,
    pub children: Vec<Node>,
}

// <li>...</li>
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListItem {
    pub children: Vec<Node>,
}

// <em>...</em>
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Emphasis {
    pub children: Vec<Node>,
}

// <strong>...</strong>
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Strong {
    pub children: Vec<Node>,
}

// <del>...</del>
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Strikethrough {
    pub children: Vec<Node>,
}

// <img src="..." title="..." />
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Image {
    pub src: String,
    pub alt: String,
    pub title: Option<String>,
}

// <img src="..." title="..." />
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HyperLink {
    pub href: String,
    pub title: Option<String>,

    pub children: Vec<Node>,
}

//#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

// impl fmt::Debug, Serialize, Deserialize, PartialEq, Eq for Table {
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
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Checkbox {
    pub checked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

impl Document {
    pub fn to_text(&self) -> String {
        let children: Vec<String> = self.children.iter().map(|m| m.to_text()).collect();

        children.join(" ")
    }
}

impl Node {
    pub fn to_text(&self) -> String {
        match self {
            Self::Text(text) => format!(" {} ", text),
            Self::Code(code) => {
                format!(" {} ", code)
            }
            Self::Html(_) => {
                panic!("Html is not supported for now!");
            }
            Self::Paragraph(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }
            Self::Heading(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }
            // Table(Table) =>{}

            // \n
            Self::SoftBreak => "\n".into(),

            // <br />
            Self::HardBreak => " ".to_string(),

            // <hr />
            Self::Rule => " ".to_string(),

            Self::Checkbox(_checkbox) => " ".to_string(),

            Self::Blockquote(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }
            Self::CodeBlock(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }
            Self::List(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }
            Self::ListItem(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }

            Self::Emphasis(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }
            Self::Strong(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }
            Self::Strikethrough(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }

            Self::Image(_p) => " ".to_string(),

            Self::HyperLink(p) => {
                let children: Vec<String> = p.children.iter().map(|m| m.to_text()).collect();

                children.join(" ")
            }
        }
    }
}
