use styling::ThemeKind;
use yew_feather::moon::Moon;
use yew_feather::sun::Sun;

use crate::prelude::*;

#[styled_component(ThemeToggle)]
pub(crate) fn theme_toggle() -> Html {
    let theme = use_theme();

    let theme_icon = match &theme.kind() {
        ThemeKind::Light => html! {<Moon size=24 />},
        ThemeKind::Dark => html! {<Sun size=24 />},
    };

    let alt_text = match &theme.kind() {
        ThemeKind::Light => "Switch to Dark Theme",
        ThemeKind::Dark => "Switch to Light Theme",
    };

    let toggle_theme = Callback::from(move |_| theme.set(theme.kind().alternative()));

    html! {
        <div
            class={css!(r#"
                height: 60px;
                width: 60px;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;
                cursor: pointer;
            "#)}
            onclick={toggle_theme}
            alt={alt_text}
        >
            {theme_icon}
        </div>
    }
}
