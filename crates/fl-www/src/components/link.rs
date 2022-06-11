use styling::{use_style, Colour};

use crate::prelude::*;

type AppLink = yew_router::prelude::Link<AppRoute>;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct LinkProps {
    pub children: Children,
    pub to: AppRoute,

    #[prop_or_default]
    pub colour: Option<Colour>,

    #[prop_or(false)]
    pub styled: bool,
}

#[styled_component(Link)]
pub(crate) fn link(props: &LinkProps) -> Html {
    let route = props.to.clone();
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
        colour = css_var!(theme.colour.primary),
        hover_colour = css_var!(theme.colour.primary_hover),
    );

    let unstyled = use_style!(
        r#"
            text-decoration: none;
            color: ${colour};
        "#,
        colour = props
            .colour
            .as_ref()
            .map(|m| m.to_string())
            .unwrap_or(css_var!(theme.colour.text.primary))
    );

    let style = if props.styled { styled } else { unstyled };

    html! {
        <AppLink to={route} classes={classes!(style)}>{children}</AppLink>
    }
}
