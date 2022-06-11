use agents::highlight::HighlightOutput;
use components::CodeSpan;

use super::ToHtml;
use crate::prelude::*;

impl ToHtml for HighlightOutput {
    fn to_html(&self) -> Html {
        let mut nodes = Vec::new();

        for (colour, s) in self.fragments.iter() {
            nodes.push(html! {<CodeSpan colour={colour}>{s}</CodeSpan>})
        }

        html! {<>{nodes}</>}
    }
}
