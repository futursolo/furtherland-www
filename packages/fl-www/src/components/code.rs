use crate::prelude::*;

#[styled_component(Code)]
pub(crate) fn code(props: &ChildrenProps) -> Html {
    let theme = use_theme();

    let children = props.children.clone();
    html! {
        <code class={css!(
            r#"
                background-color: ${bg_colour};
                padding-left: 2px;
                padding-right:2px;
                border-radius: 2px;
            "#,
            bg_colour = css_var!(theme.colour.background.code)
        )}>
            {children}
        </code>
    }
}
