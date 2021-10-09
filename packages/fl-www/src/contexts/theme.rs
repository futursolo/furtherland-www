use std::ops::Deref;

use crate::prelude::*;

use hooks::use_event;

use styling::{use_media_query, Global, Theme, ThemeKind};

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
            transition: background-color 0.3s, color 0.3s;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
        }
    "#,
        background_color = theme.colour.background.default,
        font_color = theme.colour.text.primary,
        font_family = &theme.font_family,
        font_size = &theme.font_size.root,
    );
    html! {<Global css={style} />}
}

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

            || {}
        },
        (theme_kind.clone(), prefer_dark_theme),
    );

    let children = props.children.clone();

    let state = ThemeState { inner: theme_kind };

    html! {
        <ContextProvider<ThemeState> context={state}>
            <GlobalStyle />
            {children}
        </ContextProvider<ThemeState>>
    }
}
