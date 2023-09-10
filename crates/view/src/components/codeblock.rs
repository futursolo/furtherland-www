use bounce::*;
use misc::ToHtml;
use styling::ThemeKind;
use yew::suspense::SuspensionResult;

use crate::api::{Bridge, HighlightInput, HighlightQuery};
use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct CodeBlockProps {
    pub language: Option<String>,

    pub content: String,
}

#[hook]
pub(crate) fn use_highlight(
    content: String,
    language: Option<String>,
    theme_kind: ThemeKind,
) -> SuspensionResult<Option<Html>> {
    let set_error = use_atom_setter::<ErrorState>();

    let input = match language.as_ref().cloned() {
        Some(m) => HighlightInput {
            content: content.clone(),
            language: m,
            theme_kind,
        },
        None => return Ok(None),
    };

    let highlight_query = Bridge::use_query::<HighlightQuery>(input.into())?;

    match highlight_query.as_deref() {
        Ok(m) => Ok(m.value.as_ref().map(|m| m.to_html())),
        Err(_) => {
            set_error(ErrorKind::Server.into());

            Ok(None)
        }
    }
}

#[styled_component(CodeBlock)]
pub(crate) fn code_block(props: &CodeBlockProps) -> HtmlResult {
    let theme = use_theme();

    let hl_html = use_highlight(props.content.clone(), props.language.clone(), theme.kind());

    let children = hl_html?.unwrap_or_else(|| props.content.as_str().into());

    Ok(html! {
        <pre class={css!(
            r#"
                background-color: ${bg_colour};
                padding: 20px;
                box-sizing: border-box;
                border-radius: 3px;
                transition: 0.3s background-color;

                overflow-x: auto;
            "#,
            bg_colour = css_var!(theme.colour.background.code),
        )}>
            <code>
                {children}
            </code>
        </pre>
    })
}
