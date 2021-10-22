use std::ops::Deref;

use super::Script;
use crate::prelude::*;
use hooks::use_event;
use styling::{use_media_query, CssVariables, Global, Theme, ThemeKind};
// use utils::is_ssr;

static STORAGE_KEY: &str = "fl_theme";

fn get_theme_kind() -> ThemeKind {
    if let Some(stor) = window().local_storage().ok().and_then(|m| m) {
        if let Some(kind) = stor
            .get_item(STORAGE_KEY)
            .ok()
            .and_then(|m| m)
            .and_then(|m| m.parse::<ThemeKind>().ok())
        {
            return kind;
        }

        stor.remove_item(STORAGE_KEY)
            .expect("Failed to remove item.");
    }

    ThemeKind::current()
}

fn set_theme_kind(kind: Option<ThemeKind>) {
    if let Some(stor) = window().local_storage().ok().and_then(|m| m) {
        if let Some(ref m) = kind {
            stor.set_item(STORAGE_KEY, m.as_str())
                .expect("Failed to set item.");
        } else {
            stor.remove_item(STORAGE_KEY)
                .expect("Failed to remove item.");
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ThemeState {
    inner: UseEqualStateHandle<ThemeKind>,
}

impl ThemeState {
    pub fn set(&self, next_kind: ThemeKind) {
        set_theme_kind(Some(next_kind));
        self.inner.set(next_kind);
    }

    pub fn kind(&self) -> ThemeKind {
        *self.inner.borrow()
    }
}

impl Deref for ThemeState {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        Theme::from_kind(&self.kind())
    }
}

pub(crate) fn use_theme() -> ThemeState {
    use_context::<ThemeState>().unwrap()
}

#[styled_component(GlobalStyle)]
fn global_style() -> Html {
    let theme = use_theme();

    let style = css!(
        r#"

        html, body {
            min-height: 100vh;
            margin: 0;
            background-color: ${background_color};
            color: ${font_color};
            font-family: ${font_family};
            font-size: ${font_size};
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
            transition: background-color 0.3s, color 0.3s;
        }
    "#,
        background_color = css_var!(theme.colour.background.default),
        font_color = css_var!(theme.colour.text.primary),
        font_family = &theme.font_family,
        font_size = &theme.font_size.root,
    );

    html! {
        <>
            <Global css={Theme::light().to_css_vars_for("html[data-theme='light']")} />
            <Global css={Theme::dark().to_css_vars_for("html[data-theme='dark']")} />
            <Global css={style} />
        </>
    }
}

static THEME_DETECT_SCRIPT: &str = r#"
(() => {
    let theme = localStorage.getItem("fl_theme");

    if (theme === "light") {
        document.documentElement.setAttribute("data-theme", "light");
    } else if (theme === "dark") {
        document.documentElement.setAttribute("data-theme", "dark");
    } else {
        localStorage.removeItem("fl_theme");
        document.documentElement.removeAttribute("data-theme");
    }
})();
"#;

#[function_component(ThemeProvider)]
pub(crate) fn theme_provider(props: &ChildrenProps) -> Html {
    let theme_kind = use_equal_state(get_theme_kind);

    let theme_kind_clone = theme_kind.clone();
    use_event(&window(), "storage", move |_event| {
        let next_theme_kind = get_theme_kind();

        theme_kind_clone.set(next_theme_kind);
    });

    let prefer_dark_theme = use_media_query("(prefers-color-scheme: dark)");

    use_effect_with_deps(
        |(theme_kind, _)| {
            let next_theme_kind = get_theme_kind();

            theme_kind.set(next_theme_kind);

            document()
                .document_element()
                .expect_throw("failed to get html element.")
                .set_attribute("data-theme", next_theme_kind.as_str())
                .expect_throw("failed to set theme.");
            || {}
        },
        (theme_kind.clone(), prefer_dark_theme),
    );

    let children = props.children.clone();

    let state = ThemeState { inner: theme_kind };

    html! {
        <ContextProvider<ThemeState> context={state}>
            <GlobalStyle />
            <Script content={THEME_DETECT_SCRIPT.to_string()} type_="text/javascript" />
            {children}
        </ContextProvider<ThemeState>>
    }
}
