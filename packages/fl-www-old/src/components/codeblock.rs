use crate::prelude::*;

#[cfg(not(debug_assertions))]
use misc::highlight::{HighlightInput, HighlightOutput};

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct CodeBlockProps {
    pub language: Option<String>,

    pub content: String,
}

#[styled_component(CodeBlock)]
pub(crate) fn code_block(props: &CodeBlockProps) -> Html {
    let theme = use_theme();

    let hl_html = use_state(|| -> Option<Html> { None });

    let children = (*hl_html)
        .clone()
        .unwrap_or_else(|| props.content.as_str().into());

    #[cfg(not(debug_assertions))]
    use_effect(|| {
        if let Some(m) = props.language {
            spawn_local(async move {
                let high_lighted = HighlightOutput::new(HighlightInput {
                    content,
                    language,
                    theme_kind,
                })
                .await;

                hl_html.set(high_lighted);
            })
        }

        || {}
    });

    html! {
        <pre class={css!(
            r#"
                background-color: ${bg_colour};
                padding: 20px;
                box-sizing: border-box;
                border-radius: 3px;

                overflow-x: auto;
            "#,
            bg_colour = theme.colour.background.code,
        )}>
            <code>
                {children}
            </code>
        </pre>
    }
}
