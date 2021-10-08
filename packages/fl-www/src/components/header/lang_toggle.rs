use yew_feather::globe::Globe;

use crate::prelude::*;
use components::Link;
use styling::Colour;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct LangToggleProps {
    pub colour: Colour,
}

#[styled_component(LangToggle)]
pub(crate) fn lang_toggle(props: &LangToggleProps) -> Html {
    let lang = use_language();

    let current_route = AppRoute::current_route()
        .unwrap_or_default()
        .with_lang(lang);

    html! {
        <Link to={current_route}>
            <div class={css!(
                r#"
                    height: 60px;
                    width: 60px;
                    color: ${colour};

                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;
                    cursor: pointer;
                "#,
                colour = props.colour,
            )}>
                <Globe size={24} />
            </div>
        </Link>
    }
}
