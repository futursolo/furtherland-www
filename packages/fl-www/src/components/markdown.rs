use pulldown_cmark::Parser;

use crate::prelude::*;
use misc::markdown::HtmlCreator;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct MarkdownProps {
    pub markdown_text: String,
}

#[function_component(Markdown)]
pub(crate) fn markdown(props: &MarkdownProps) -> Html {
    let children = HtmlCreator::new(Parser::new(&props.markdown_text)).into_html();
    html! {
        <div>{children}</div>
    }
}
