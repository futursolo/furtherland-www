use std::ops::Deref;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use super::Script;
use crate::prelude::*;
use hooks::use_event;
use styling::{use_media_query, CssVariables, Global, Theme, ThemeKind};
// use utils::is_ssr;

static STORAGE_KEY: &str = "fl_theme";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PersistedThemeState {
    kind: ThemeKind,
    last_updated: i64,
}

impl From<ThemeKind> for PersistedThemeState {
    fn from(kind: ThemeKind) -> Self {
        Self {
            kind,
            last_updated: Utc::now().timestamp(),
        }
    }
}

fn get_theme_kind() -> ThemeKind {
    let persisted_state: Option<PersistedThemeState> = LocalStorage::get(STORAGE_KEY).ok();

    if let Some(m) = persisted_state {
        let updated =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(m.last_updated, 0), Utc);

        // theme selection is persisted for 6 hours.
        if Utc::now() - updated <= Duration::hours(6) {
            return m.kind;
        }
    }

    LocalStorage::delete(STORAGE_KEY);
    ThemeKind::current()
}

fn set_theme_kind(kind: Option<ThemeKind>) {
    match kind {
        Some(m) => {
            let persisted_state = PersistedThemeState::from(m);
            LocalStorage::set(STORAGE_KEY, persisted_state).expect_throw("failed to set theme.");
        }
        None => LocalStorage::delete(STORAGE_KEY),
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

    if (theme) {
        try {
            let themeState = JSON.parse(theme);

            if ((Date.now() / 1000) - themeState.last_updated <= 6 * 60 * 60) {
                let themeKind = themeState.kind;

                if (themeKind === "light") {
                    document.documentElement.setAttribute("data-theme", "light");

                    return;
                }
                if (themeKind === "dark") {
                    document.documentElement.setAttribute("data-theme", "dark");

                    return;
                }

            }
        } catch (_e) {}
    }

    localStorage.removeItem("fl_theme");

    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
        document.documentElement.setAttribute("data-theme", "dark");
    } else {
        document.documentElement.setAttribute("data-theme", "light");
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
