use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    pub children: Children,
    #[prop_or(3.0)]
    pub font_size: f32,
}

#[styled_component(SectionTitle)]
pub(crate) fn section_title(props: &Props) -> Html {
    let children = props.children.clone();
    html! {
        <div class={css!(r#"
            display: flex;
            width: 100%;

            flex-direction: column;
            align-items: flex-start;
            justify-content: flex-start;
        "#)}>
            <h1 class={css!("font-size: ${font_size}rem;", font_size = props.font_size)}>
                {children}
            </h1>
        </div>
    }
}
