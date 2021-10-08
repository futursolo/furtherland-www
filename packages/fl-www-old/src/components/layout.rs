use crate::prelude::*;

#[styled_component(Layout)]
pub(crate) fn layout(props: &ChildrenProps) -> Html {
    let children = props.children.clone();
    html! {
        <div class={css!(r#"
            display: flex;
            width: 100%;
            min-height: 100vh;

            flex-direction: column;
            align-items: center;
            justify-content: flex-start;
        "#)}>{children}</div>
    }
}
