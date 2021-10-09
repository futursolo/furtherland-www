use std::collections::HashMap;
use std::sync::Arc;

use futures::lock::Mutex;
use gloo::timers::future::TimeoutFuture;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use stylist::Style;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as Colours, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::prelude::*;
use styling::ThemeKind;

static SYNTAX_SET: Lazy<SyntaxSet> = macros::fl_syntax_set!();
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

static CACHE: Lazy<Arc<Mutex<HashMap<HighlightInput, HighlightOutput>>>> = Lazy::new(Arc::default);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HighlightInput {
    pub content: String,
    pub language: String,
    pub theme_kind: ThemeKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HighlightOutput {
    fragments: Vec<(Colours, String)>,
}

impl HighlightOutput {
    // This method is incredibly slow in debug mode.
    // So needs to be paused.
    pub async fn new<'a>(input: HighlightInput) -> Option<HighlightOutput> {
        let mut cache = CACHE.lock().await;

        if let Some(m) = cache.get(&input).cloned() {
            return Some(m);
        }

        TimeoutFuture::new(1).await;
        let syntax_ref = SYNTAX_SET.find_syntax_by_token(&input.language)?;

        TimeoutFuture::new(1).await;
        let theme_name = match input.theme_kind {
            ThemeKind::Light => "base16-ocean.light",
            ThemeKind::Dark => "base16-ocean.dark",
        };
        let mut h = HighlightLines::new(syntax_ref, &THEME_SET.themes[theme_name]);

        TimeoutFuture::new(1).await;
        let mut fragments = Vec::new();

        for line in LinesWithEndings::from(&input.content) {
            for frag in h.highlight(line, &SYNTAX_SET) {
                fragments.push((frag.0, frag.1.to_string()));
            }
        }

        let self_ = Self { fragments };

        cache.insert(input, self_.clone());

        Some(self_)
    }

    pub fn to_html(&self) -> Html {
        let mut nodes = Vec::new();

        for (colours, s) in self.fragments.iter() {
            /* background-color: rgb({}, {}, {}); */
            let style = Style::new(format!(
                r#"
                    color: rgb({}, {}, {});
                "#,
                colours.foreground.r,
                colours.foreground.g,
                colours.foreground.b,
                // colours.background.r,
                // colours.background.g,
                // colours.background.b,
            ))
            .unwrap();

            nodes.push(html! {<span class={style}>{s}</span>})
        }

        html! {<>{nodes}</>}
    }
}
