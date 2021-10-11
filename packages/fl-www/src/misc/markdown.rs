// Based on pulldown_cmark's HtmlWriter.

use crate::prelude::*;
use agents::markdown::*;
use components::{Code, CodeBlock as CompCodeBlock, Hyperlink, SectionTitle};
use yew::prelude::Html;

use super::ToHtml;

impl ToHtml for Root {
    fn to_html(&self) -> Html {
        let children: Vec<Html> = self.nodes.iter().map(|m| m.to_html()).collect();

        html! {<>{children}</>}
    }
}

impl ToHtml for Node {
    fn to_html(&self) -> Html {
        match self {
            Self::Text(text) => text.into(),
            Self::Code(code) => {
                html! {<Code>{code}</Code>}
            }
            Self::Html(_) => {
                panic!("Html is not supported for now!");
            }
            Self::Paragraph(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();
                html! {<p>{children}</p>}
            }
            Self::Heading(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();
                if p.level == 1 {
                    html! {<SectionTitle>{children}</SectionTitle>}
                } else {
                    html! {<@{format!("h{}", p.level)}>{children}</@>}
                }
            }
            // Table(Table) =>{}

            // \n
            Self::SoftBreak => "\n".into(),

            // <br />
            Self::HardBreak => {
                html! {<br />}
            }

            // <hr />
            Self::Rule => {
                html! {<hr />}
            }

            Self::Checkbox(checkbox) => {
                html! {<input type_="checkbox" disabled={true} checked={checkbox.checked} />}
            }

            Self::Blockquote(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();
                html! {<blockquote>{children}</blockquote>}
            }
            Self::CodeBlock(p) => {
                let children: String = p
                    .children
                    .iter()
                    .map(|m| match m {
                        Self::Text(m) => m.as_str(),
                        _ => panic!(),
                    })
                    .collect();

                html! {<CompCodeBlock language={p.language.clone()} content={children} />}
            }
            Self::List(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();

                if let Some(m) = p.start {
                    let start = if m == 1 { None } else { Some(m.to_string()) };

                    html! {<ol start={start}>{children}</ol>}
                } else {
                    html! {<ul>{children}</ul>}
                }
            }
            Self::ListItem(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();
                html! {<li>{children}</li>}
            }

            Self::Emphasis(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();
                html! {<em>{children}</em>}
            }
            Self::Strong(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();
                html! {<strong>{children}</strong>}
            }
            Self::Strikethrough(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();
                html! {<del>{children}</del>}
            }

            Self::Image(p) => {
                html! {<img src={p.src.clone()} title={p.title.clone()} alt={p.alt.clone()} />}
            }

            Self::HyperLink(p) => {
                let children: Html = p.children.iter().map(|m| m.to_html()).collect();
                html! {<Hyperlink styled={true} href={p.href.clone()} title={p.title.clone()}>{children}</Hyperlink>}
            }
        }
    }
}
