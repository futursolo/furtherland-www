// Based on pulldown_cmark's HtmlWriter.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default, Debug)]
pub(crate) struct Root {
    pub children: Vec<Node>,
}

impl From<Root> for crate::types::Document {
    fn from(value: Root) -> Self {
        let Root { children, .. } = value;

        Self {
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <pre><code>...</code></pre>
#[derive(Clone, Default, Debug)]
pub(crate) struct CodeBlock {
    pub language: Option<String>,
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<CodeBlock> for crate::types::CodeBlock {
    fn from(value: CodeBlock) -> Self {
        let CodeBlock {
            language, children, ..
        } = value;

        Self {
            language,
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <p>...</p>
#[derive(Clone, Default, Debug)]
pub(crate) struct Paragraph {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<Paragraph> for crate::types::Paragraph {
    fn from(value: Paragraph) -> Self {
        let Paragraph { children, .. } = value;

        Self {
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <h1 - h6>...</h1 - h6>
#[derive(Clone, Default, Debug)]
pub(crate) struct Heading {
    pub level: u32,
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<Heading> for crate::types::Heading {
    fn from(value: Heading) -> Self {
        let Heading {
            level, children, ..
        } = value;

        Self {
            level,
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <blockquote>...</blockquote>
#[derive(Clone, Default, Debug)]
pub(crate) struct Blockquote {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<Blockquote> for crate::types::Blockquote {
    fn from(value: Blockquote) -> Self {
        let Blockquote { children, .. } = value;

        Self {
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// no start: ul, Unordered List
// start: ol, Ordered List
#[derive(Clone, Default, Debug)]
pub(crate) struct List {
    pub start: Option<u64>,
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<List> for crate::types::List {
    fn from(value: List) -> Self {
        let List {
            start, children, ..
        } = value;

        Self {
            start,
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <li>...</li>
#[derive(Clone, Default, Debug)]
pub(crate) struct ListItem {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<ListItem> for crate::types::ListItem {
    fn from(value: ListItem) -> Self {
        let ListItem { children, .. } = value;

        Self {
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <em>...</em>
#[derive(Clone, Default, Debug)]
pub(crate) struct Emphasis {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<Emphasis> for crate::types::Emphasis {
    fn from(value: Emphasis) -> Self {
        let Emphasis { children, .. } = value;

        Self {
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <strong>...</strong>
#[derive(Clone, Default, Debug)]
pub(crate) struct Strong {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<Strong> for crate::types::Strong {
    fn from(value: Strong) -> Self {
        let Strong { children, .. } = value;

        Self {
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <del>...</del>
#[derive(Clone, Default, Debug)]
pub(crate) struct Strikethrough {
    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<Strikethrough> for crate::types::Strikethrough {
    fn from(value: Strikethrough) -> Self {
        let Strikethrough { children, .. } = value;

        Self {
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
}

// <img src="..." title="..." />
#[derive(Clone, Default, Debug)]
pub(crate) struct Image {
    pub src: String,
    pub alt: String,
    pub title: Option<String>,
}

impl From<Image> for crate::types::Image {
    fn from(value: Image) -> Self {
        let Image { src, alt, title } = value;

        Self { src, alt, title }
    }
}

// <img src="..." title="..." />
#[derive(Clone, Default, Debug)]
pub(crate) struct HyperLink {
    pub href: String,
    pub title: Option<String>,

    pub children: Vec<Node>,
    pub parent: Rc<RefCell<Option<Node>>>,
}

impl From<HyperLink> for crate::types::HyperLink {
    fn from(value: HyperLink) -> Self {
        let HyperLink {
            href,
            title,
            children,
            ..
        } = value;

        Self {
            href,
            title,
            children: children.into_iter().map(|m| m.into()).collect(),
        }
    }
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
pub(crate) struct Checkbox {
    pub checked: bool,
}

impl From<Checkbox> for crate::types::Checkbox {
    fn from(value: Checkbox) -> Self {
        Self {
            checked: value.checked,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Node {
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

impl From<Node> for crate::types::Node {
    fn from(value: Node) -> Self {
        match value {
            Node::Text(m) => Self::Text(m),
            Node::Code(m) => Self::Code(m),
            Node::Html(m) => Self::Html(m),
            Node::Paragraph(m) => Self::Paragraph(m.into()),
            Node::Heading(m) => Self::Heading(m.into()),
            // Table(Table),

            // \n
            Node::SoftBreak => Self::SoftBreak,

            // <br />
            Node::HardBreak => Self::HardBreak,

            // <hr />
            Node::Rule => Self::Rule,

            Node::Checkbox(m) => Self::Checkbox(m.into()),

            Node::Blockquote(m) => Self::Blockquote(m.into()),
            Node::CodeBlock(m) => Self::CodeBlock(m.into()),
            Node::List(m) => Self::List(m.into()),
            Node::ListItem(m) => Self::ListItem(m.into()),

            Node::Emphasis(m) => Self::Emphasis(m.into()),
            Node::Strong(m) => Self::Strong(m.into()),
            Node::Strikethrough(m) => Self::Strikethrough(m.into()),

            Node::Image(m) => Self::Image(m.into()),

            Node::HyperLink(m) => Self::HyperLink(m.into()),
        }
    }
}
