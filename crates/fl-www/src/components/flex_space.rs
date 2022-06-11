use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct FlexSpaceProps {
    #[prop_or(1)]
    pub scale: u32,
}

#[styled_component(FlexSpace)]
pub(crate) fn flex_space(props: &FlexSpaceProps) -> Html {
    html! {
        <div class={css!(
            r#"
                flex-grow: ${scale};
            "#,
            scale = props.scale
        )} />
    }
}
