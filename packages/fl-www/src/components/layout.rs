use crate::prelude::*;

#[styled_component(Layout)]
pub(crate) fn layout(props: &ChildrenProps) -> Html {
    let children = props.children.clone();
    let theme = use_theme();

    html! {
        <div class={css!(
            r#"
                display: flex;
                width: 100%;
                min-height: 100vh;

                flex-direction: column;
                align-items: center;
                justify-content: flex-start;

                background-color: ${background_color};
                color: ${font_color};
            "#,
            background_color = theme.colour.background.default,
            font_color = theme.colour.text.primary,
        )}>{children}</div>
    }
}
