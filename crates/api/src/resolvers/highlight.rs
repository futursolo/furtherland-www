use std::time::Duration;

use async_trait::async_trait;
use once_cell::sync::Lazy;
use prokio::time::sleep;
use stellation_bridge::resolvers::QueryResolver;
use stellation_bridge::routines::QueryResult;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::core::styling::{Colour, ThemeKind};
use crate::routines::HighlightQuery;
use crate::{HighlightInput, HighlightOutput, ResolverContext};

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

impl HighlightOutput {
    // This method is incredibly slow in debug mode.
    // So needs to be paused.
    pub async fn new<'a>(input: &HighlightInput) -> Option<HighlightOutput> {
        sleep(Duration::ZERO).await;
        let syntax_ref = SYNTAX_SET.find_syntax_by_token(&input.language)?;

        sleep(Duration::ZERO).await;
        let theme_name = match input.theme_kind {
            ThemeKind::Light => "base16-ocean.light",
            ThemeKind::Dark => "base16-ocean.dark",
        };
        let mut h = HighlightLines::new(syntax_ref, &THEME_SET.themes[theme_name]);

        sleep(Duration::ZERO).await;
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

            sleep(Duration::ZERO).await;
        }

        let self_ = Self { fragments };

        // cache.insert(input, self_.clone());

        Some(self_)
    }
}

#[async_trait(?Send)]
impl QueryResolver for HighlightQuery {
    type Context = ResolverContext;

    async fn resolve(_ctx: &ResolverContext, input: &Self::Input) -> QueryResult<Self> {
        let output = HighlightOutput::new(input).await;

        Ok(Self { value: output }.into())
    }
}
