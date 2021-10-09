use crate::prelude::*;

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

    use_effect_with_deps(
        move |(content, language, theme_kind)| {
            let theme_kind = *theme_kind;
            let language = language.to_owned();
            let content = content.to_owned();

            if let Some(m) = language {
                spawn_local(async move {
                    let high_lighted = HighlightOutput::new(HighlightInput {
                        content,
                        language: m,
                        theme_kind,
                    })
                    .await
                    .map(|m| m.to_html());

                    hl_html.set(high_lighted);
                })
            }

            || {}
        },
        (props.content.clone(), props.language.clone(), theme.kind()),
    );

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
