// use std::collections::HashMap;
// use std::sync::Arc;

// use futures::lock::Mutex;
// use std::collections::HashMap;

use gloo::timers::future::TimeoutFuture;
use once_cell::sync::Lazy;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::prelude::*;
use styling::{Colour, ThemeKind};

use super::{HighlightInput, HighlightOutput};

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

// static CACHE: Lazy<Arc<Mutex<HashMap<HighlightInput, HighlightOutput>>>> = Lazy::new(Arc::default);

impl HighlightOutput {
    // This method is incredibly slow in debug mode.
    // So needs to be paused.
    pub async fn new<'a>(input: HighlightInput) -> Option<HighlightOutput> {
        // let mut cache = CACHE.lock().await;

        // if let Some(m) = cache.get(&input).cloned() {
        //     return Some(m);
        // }

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
            for frag in h
                .highlight_line(line, &SYNTAX_SET)
                .expect("failed tob highlight syntax")
            {
                let colour = Colour::from_rgb(
                    frag.0.foreground.r,
                    frag.0.foreground.g,
                    frag.0.foreground.b,
                );

                fragments.push((colour, frag.1.to_string()));
            }

            TimeoutFuture::new(1).await;
        }

        let self_ = Self { fragments };

        // cache.insert(input, self_.clone());

        Some(self_)
    }
}
