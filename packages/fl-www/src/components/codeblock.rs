use crate::prelude::*;
use agents::highlight::{HighlightInput, HighlightOutput};
use agents::prelude::*;
use atoms::CacheState;
use misc::ToHtml;
use styling::ThemeKind;

use bounce::use_slice;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct CodeBlockProps {
    pub language: Option<String>,

    pub content: String,
}

pub(crate) fn use_highlight(
    content: String,
    language: Option<String>,
    theme_kind: ThemeKind,
) -> Option<Html> {
    let cache_state = use_slice::<CacheState>();

    let input = language.as_ref().cloned().map(|m| HighlightInput {
        content: content.clone(),
        language: m,
        theme_kind,
    });

    let hl_html = {
        let cache_state = cache_state.clone();
        let input = input.clone();

        use_state_eq(move || -> Option<Html> {
            input.and_then(|m| {
                cache_state
                    .get::<HighlightInput, Option<HighlightOutput>>(&m)
                    .flatten()
                    .map(|m| m.to_html())
            })
        })
    };

    let worker = {
        let hl_html = hl_html.clone();
        let input = input.clone();
        let cache_state = cache_state.clone();
        use_bridge::<agents::highlight::Worker, _>(move |m| {
            let agents::highlight::Response::Highlighted(m) = m;

            let action = CacheState::convert_action::<HighlightInput, Option<HighlightOutput>>(
                input.as_ref().unwrap(),
                m.clone(),
            )
            .unwrap_throw();
            cache_state.dispatch(action);

            if let Some(m) = m.map(|m| m.to_html()) {
                hl_html.set(Some(m));
            }
        })
    };

    {
        let hl_html = hl_html.clone();
        use_effect_with_deps(
            move |input| {
                if let Some(ref m) = input {
                    if let Some(cached) = cache_state
                        .get::<HighlightInput, Option<HighlightOutput>>(m)
                        .flatten()
                        .map(|m| m.to_html())
                    {
                        hl_html.set(Some(cached));
                    } else {
                        worker.send(agents::highlight::Request::Highlight(m.clone()));
                    }
                } else {
                    hl_html.set(None);
                }

                || {}
            },
            input,
        );
    }

    (*hl_html).clone()
}

#[styled_component(CodeBlock)]
pub(crate) fn code_block(props: &CodeBlockProps) -> Html {
    let theme = use_theme();

    let hl_html = use_highlight(props.content.clone(), props.language.clone(), theme.kind());

    let status = if props.language.is_some() && hl_html.is_none() {
        "loading"
    } else {
        "done"
    };

    let children = hl_html.unwrap_or_else(|| props.content.as_str().into());

    html! {
        <pre data-status={status} class={css!(
            r#"
                background-color: ${bg_colour};
                padding: 20px;
                box-sizing: border-box;
                border-radius: 3px;

                overflow-x: auto;
            "#,
            bg_colour = css_var!(theme.colour.background.code),
        )}>
            <code>
                {children}
            </code>
        </pre>
    }
}
