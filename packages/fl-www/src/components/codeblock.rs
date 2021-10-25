use std::cell::RefCell;

use crate::prelude::*;
use agents::highlight::HighlightInput;
use misc::ToHtml;
use styling::ThemeKind;
use yew_agent::Bridged;

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
    let hl_html = use_state_eq(|| -> Option<Html> { None });

    let hl_html_clone = hl_html.clone();
    let worker = use_state(move || {
        RefCell::new(agents::highlight::Worker::bridge(Callback::from(
            move |m| {
                let agents::highlight::Response::Highlighted(m) = m;

                if let Some(m) = m.map(|m| m.to_html()) {
                    hl_html_clone.set(Some(m));
                }
            },
        )))
    });

    use_effect_with_deps(
        move |(content, language, theme_kind)| {
            let content = content.clone();
            if let Some(m) = language {
                let theme_kind = *theme_kind;

                let input = HighlightInput {
                    content,
                    language: m.to_owned(),
                    theme_kind,
                };

                worker
                    .borrow_mut()
                    .send(agents::highlight::Request::Highlight(input));
            }

            || {}
        },
        (content, language, theme_kind),
    );

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
