use crate::prelude::*;
use styling::Colour;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct CodeSpanProps {
    pub children: Children,
    pub colour: Colour,
}

#[styled_component(CodeSpan)]
pub(crate) fn code_span(props: &CodeSpanProps) -> Html {
    let children = props.children.clone();

    html! {
        <span class={css!(
            r#"
                color: ${colour};
            "#,
            colour = props.colour
        )}>
            {children}
        </span>
    }
}
