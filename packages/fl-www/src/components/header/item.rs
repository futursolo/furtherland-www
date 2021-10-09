use crate::prelude::*;
use styling::{use_style, Colour};

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct ItemProps {
    pub children: Children,
    pub colour: Colour,
}

#[styled_component(Item)]
pub(crate) fn item(props: &ItemProps) -> Html {
    let indicator_style = use_style!(
        r#"
            height: 3px;
            width: 0%;
            transition: width 0.2s ease-out;
            background-color: ${bg_colour};
        "#,
        bg_colour = props.colour
    );

    html! {
        <div class={css!(
            r#"
                height: 60px;
                font-size: 1.1rem;
                font-weight: bold;
                padding-left: 15px;
                padding-right: 15px;

                transition: color 0.3s;

                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                &:hover .${indicator_class_name} {
                    width: 100%;
                }
            "#,
            indicator_class_name = indicator_style.get_class_name()
        )}>
            <div class={css!(r#"
                flex-grow: 1;
                line-height: 57px;
            "#)}>{props.children.clone()}</div>
            <div class={indicator_style} />
        </div>
    }
}
