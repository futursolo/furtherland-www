use crate::prelude::*;
use styling::{use_style, Colour};

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct HyperlinkProps {
    pub children: Children,
    pub href: String,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub colour: Option<Colour>,

    #[prop_or(false)]
    pub styled: bool,
}

#[styled_component(Hyperlink)]
pub(crate) fn hyperlink(props: &HyperlinkProps) -> Html {
    let children = props.children.clone();
    let theme = use_theme();

    let styled = use_style!(
        r#"
            color: ${colour};
            transition: color 0.3s;
            text-decoration: none;

            &:hover {
                color: ${hover_colour};
                text-decoration: underline;
            }
        "#,
        colour = theme.colour.primary,
        hover_colour = theme.colour.primary_hover,
    );

    let unstyled = use_style!(
        r#"
            text-decoration: none;
            color: ${colour};
        "#,
        colour = props.colour.as_ref().unwrap_or(&theme.colour.text.primary)
    );

    let style = if props.styled { styled } else { unstyled };

    html! {
        <a href={props.href.clone()}
            class={style}
            title={props.title.as_ref().map(|m| m.to_string())}
        >
            {children}
        </a>
    }
}
