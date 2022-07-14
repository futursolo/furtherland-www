// Based on pulldown_cmark's HtmlWriter.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
// use std::fmt;
use std::sync::{Arc, Mutex};

// use pulldown_cmark::escape::StrWrite;
use pulldown_cmark::Event::*;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, LinkType, Tag};

use super::types::*;

pub(crate) type CowStr<'a> = Cow<'a, str>;

impl Node {
    fn add_child(&mut self, node: Node) {
        match self {
            Self::Text(_) => {
                panic!("This node cannot have child!");
            }
            Self::Code(_) => {
                panic!("This node cannot have child!");
            }
            Self::Html(_) => {
                panic!("This node cannot have child!");
            }
            Self::Paragraph(p) => {
                p.children.push(node);
            }
            Self::Heading(p) => {
                p.children.push(node);
            }
            // Table(Table) =>{}

            // \n
            Self::SoftBreak => {
                panic!("This node cannot have child!");
            }

            // <br />
            Self::HardBreak => {
                panic!("This node cannot have child!");
            }

            // <hr />
            Self::Rule => {
                panic!("This node cannot have child!");
            }

            Self::Checkbox(_) => {
                panic!("This node cannot have child!");
            }

            Self::Blockquote(p) => {
                p.children.push(node);
            }
            Self::CodeBlock(p) => {
                p.children.push(node);
            }
            Self::List(p) => {
                p.children.push(node);
            }
            Self::ListItem(p) => {
                p.children.push(node);
            }

            Self::Emphasis(p) => {
                p.children.push(node);
            }
            Self::Strong(p) => {
                p.children.push(node);
            }
            Self::Strikethrough(p) => {
                p.children.push(node);
            }

            Self::Image(_) => {
                panic!("This node cannot have child!");
            }

            Self::HyperLink(p) => {
                p.children.push(node);
            }
        }
    }
}

//#[derive(Debug)]
// enum TableState {
// Head,
// Body,
//}

#[derive(Debug)]
pub struct HtmlCreator<'a, I> {
    iter: I,

    root: Vec<Node>,
    current_node: Option<Node>,

    numbers: HashMap<CowStr<'a>, usize>,
}

impl<'a, I> HtmlCreator<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            root: Vec::new(),
            current_node: None,
            numbers: HashMap::new(),
        }
    }

    pub fn into_root_node(mut self) -> Root {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.parse_start_tag(tag);
                }
                End(tag) => {
                    self.parse_end_tag(tag);
                }
                Text(text) => {
                    let t = Node::Text(text.into_string());
                    if let Some(ref mut m) = self.current_node {
                        m.add_child(t);
                    } else {
                        self.root.push(t);
                    }
                }
                Code(text) => {
                    let t = Node::Code(text.into_string());
                    if let Some(ref mut m) = self.current_node {
                        m.add_child(t);
                    } else {
                        self.root.push(t);
                    }
                }
                Html(html) => {
                    let t = Node::Html(html.into_string());
                    if let Some(ref mut m) = self.current_node {
                        m.add_child(t);
                    } else {
                        self.root.push(t);
                    }
                }
                SoftBreak => {
                    let t = Node::SoftBreak;
                    if let Some(ref mut m) = self.current_node {
                        m.add_child(t);
                    } else {
                        self.root.push(t);
                    }
                }
                HardBreak => {
                    let t = Node::HardBreak;
                    if let Some(ref mut m) = self.current_node {
                        m.add_child(t);
                    } else {
                        self.root.push(t);
                    }
                }
                Rule => {
                    let t = Node::Rule;
                    if let Some(ref mut m) = self.current_node {
                        m.add_child(t);
                    } else {
                        self.root.push(t);
                    }
                }

                FootnoteReference(_name) => {
                    panic!("Not implemented!");
                    // let len = self.numbers.len() + 1;
                    // self.write("<sup class=\"footnote-reference\"><a href=\"#")?;
                    // escape_html(&mut self.writer, &name)?;
                    // self.write("\">")?;
                    // let number = *self.numbers.entry(name).or_insert(len);
                    // write!(&mut self.writer, "{}", number)?;
                    // self.write("</a></sup>")?;
                }
                TaskListMarker(checked) => {
                    let t = Node::Checkbox(Checkbox { checked });
                    if let Some(ref mut m) = self.current_node {
                        m.add_child(t);
                    } else {
                        self.root.push(t);
                    }
                }
            }
        }

        Root { nodes: self.root }
    }

    /// Writes the start of an HTML tag.
    fn parse_start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::Paragraph => {
                let p = Paragraph {
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    ..Paragraph::default()
                };
                self.current_node = Some(Node::Paragraph(p));
            }
            Tag::Heading(level, _, _) => {
                let level = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,

                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };

                self.current_node = Some(Node::Heading(Heading {
                    level,
                    children: Vec::new(),
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                }));
            }
            //            Tag::Table(alignments) => {
            //let mut t = Table::default();

            //t.table_alignments = alignments;
            //t.parent = Arc::new(Mutex::new(self.current_node));

            //self.current_node = Some(Node::Table(t));
            //}
            //Tag::TableHead => {
            //self.table_state = TableState::Head;
            //self.table_cell_index = 0;
            //self.write("<thead><tr>")
            //}
            //Tag::TableRow => {
            //self.table_cell_index = 0;
            //self.write("<tr>")
            //}
            //Tag::TableCell => {
            //match self.table_state {
            //TableState::Head => {
            //self.write("<th")?;
            //}
            //TableState::Body => {
            //self.write("<td")?;
            //}
            //}
            //match self.table_alignments.get(self.table_cell_index) {
            //Some(&Alignment::Left) => self.write(" align=\"left\">"),
            //Some(&Alignment::Center) => self.write(" align=\"center\">"),
            //Some(&Alignment::Right) => self.write(" align=\"right\">"),
            //_ => self.write(">"),
            //}
            //}
            Tag::BlockQuote => {
                let p = Blockquote {
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    ..Blockquote::default()
                };
                self.current_node = Some(Node::Blockquote(p));
            }
            Tag::CodeBlock(info) => {
                let mut p = CodeBlock {
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    ..CodeBlock::default()
                };

                if let CodeBlockKind::Fenced(info) = info {
                    let lang = info.split(' ').next().unwrap();
                    if !lang.is_empty() {
                        p.language = Some(lang.into());
                    }
                }

                self.current_node = Some(Node::CodeBlock(p));
            }
            Tag::List(start) => {
                let p = List {
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    start,
                    ..List::default()
                };
                self.current_node = Some(Node::List(p));
            }
            Tag::Item => {
                let p = ListItem {
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    ..ListItem::default()
                };
                self.current_node = Some(Node::ListItem(p));
            }
            Tag::Emphasis => {
                let p = Emphasis {
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    ..Emphasis::default()
                };
                self.current_node = Some(Node::Emphasis(p));
            }
            Tag::Strong => {
                let p = Strong {
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    ..Strong::default()
                };

                self.current_node = Some(Node::Strong(p));
            }
            Tag::Strikethrough => {
                let p = Strikethrough {
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    ..Strikethrough::default()
                };

                self.current_node = Some(Node::Strikethrough(p));
            }
            Tag::Link(LinkType::Email, dest, title) => {
                let p = HyperLink {
                    href: format!("mailto:{}", dest.into_string()),
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    title: Some(title.into_string()),
                    ..HyperLink::default()
                };

                self.current_node = Some(Node::HyperLink(p));
            }
            Tag::Link(_link_type, dest, title) => {
                let p = HyperLink {
                    href: dest.into_string(),
                    parent: Arc::new(Mutex::new(self.current_node.take())),
                    title: Some(title.into_string()),
                    ..HyperLink::default()
                };

                self.current_node = Some(Node::HyperLink(p));
            }
            Tag::Image(_link_type, dest, title) => {
                let p = Image {
                    src: dest.into_string(),
                    alt: self.parse_raw_text(),
                    title: Some(title.into_string()),
                };

                let p = Node::Image(p);

                if let Some(m) = self.current_node.as_mut() {
                    m.add_child(p);
                } else {
                    self.root.push(p);
                }
            }

            //Tag::FootnoteDefinition(name) => {
            //if self.end_newline {
            //self.write("<div class=\"footnote-definition\" id=\"")?;
            //} else {
            //self.write("\n<div class=\"footnote-definition\" id=\"")?;
            //}
            //escape_html(&mut self.writer, &*name)?;
            //self.write("\"><sup class=\"footnote-definition-label\">")?;
            //let len = self.numbers.len() + 1;
            //let number = *self.numbers.entry(name).or_insert(len);
            //write!(&mut self.writer, "{}", number)?;
            //self.write("</sup>")
            //}
            _ => unimplemented!("Unimplemented feature! {:?}", tag),
        }
    }

    fn parse_end_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => match self.current_node.take().unwrap() {
                Node::Paragraph(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::Paragraph(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::Paragraph(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not paragraph?"),
            },
            Tag::Heading(_level, _, _) => match self.current_node.take().unwrap() {
                // Verify level is the same?
                Node::Heading(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::Heading(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::Heading(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not heading?"),
            },

            //Tag::Table(_) => {
            //self.write("</tbody></table>\n")?;
            //}
            //Tag::TableHead => {
            //self.write("</tr></thead><tbody>\n")?;
            //self.table_state = TableState::Body;
            //}
            //Tag::TableRow => {
            //self.write("</tr>\n")?;
            //}
            //Tag::TableCell => {
            //match self.table_state {
            //TableState::Head => {
            //self.write("</th>")?;
            //}
            //TableState::Body => {
            //self.write("</td>")?;
            //}
            //}
            //self.table_cell_index += 1;
            //}
            Tag::BlockQuote => match self.current_node.take().unwrap() {
                Node::Blockquote(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::Blockquote(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::Blockquote(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not blockquote?"),
            },
            Tag::CodeBlock(_) => match self.current_node.take().unwrap() {
                Node::CodeBlock(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::CodeBlock(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::CodeBlock(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not codeblock?"),
            },
            Tag::List(_) => match self.current_node.take().unwrap() {
                Node::List(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::List(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::List(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not list?"),
            },
            Tag::Item => match self.current_node.take().unwrap() {
                Node::ListItem(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::ListItem(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::ListItem(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not list item?"),
            },
            Tag::Emphasis => match self.current_node.take().unwrap() {
                Node::Emphasis(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::Emphasis(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::Emphasis(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not emphasis?"),
            },
            Tag::Strong => match self.current_node.take().unwrap() {
                Node::Strong(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::Strong(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::Strong(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not strong?"),
            },
            Tag::Strikethrough => match self.current_node.take().unwrap() {
                Node::Strikethrough(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::Strikethrough(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::Strikethrough(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not strikethrough?"),
            },
            Tag::Link(_, _, _) => match self.current_node.take().unwrap() {
                Node::HyperLink(p) => {
                    if let Some(mut m) = p.parent.clone().lock().unwrap().take() {
                        m.add_child(Node::HyperLink(p));
                        self.current_node = Some(m);
                    } else {
                        self.root.push(Node::HyperLink(p));
                        self.current_node = None;
                    }
                }
                _ => panic!("Not hyperlink?"),
            },
            Tag::Image(_, _, _) => (),

            //Tag::FootnoteDefinition(_) => {
            //self.write("</div>\n")?;
            //}
            _ => unimplemented!("Unimplemented feature! {:?}", tag),
        }
    }

    // run raw text, consuming end tag
    fn parse_raw_text(&mut self) -> String {
        let mut s = "".to_string();
        let mut nest = 0;
        for event in &mut self.iter {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Html(text) | Code(text) | Text(text) => {
                    s.push_str(&text);
                }
                SoftBreak | HardBreak | Rule => {
                    s.push(' ');
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name.into_string().into()).or_insert(len);
                    let _ = write!(s, "[{}]", number);
                }
                TaskListMarker(true) => s.push_str("[x]"),
                TaskListMarker(false) => s.push_str("[ ]"),
            }
        }
        s
    }

    // run raw text, consuming end tag
    // fn raw_text(&mut self) -> io::Result<()> {
    // let mut nest = 0;
    // while let Some(event) = self.iter.next() {
    // match event {
    // Start(_) => nest += 1,
    // End(_) => {
    // if nest == 0 {
    // break;
    //}
    // nest -= 1;
    //}
    // Html(text) | Code(text) | Text(text) => {
    // escape_html(&mut self.writer, &text)?;
    // self.end_newline = text.ends_with('\n');
    //}
    // SoftBreak | HardBreak | Rule => {
    // self.write(" ")?;
    //}
    // FootnoteReference(name) => {
    // let len = self.numbers.len() + 1;
    // let number = *self.numbers.entry(name).or_insert(len);
    // write!(&mut self.writer, "[{}]", number)?;
    //}
    // TaskListMarker(true) => self.write("[x]")?,
    // TaskListMarker(false) => self.write("[ ]")?,
    //}
    // Ok(())
    //}
}
