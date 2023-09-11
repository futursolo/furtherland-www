use bounce::prelude::*;
use fl_www_api::{MarkdownQuery, MarkdownQueryInput};
use misc::ToHtml;

use super::Placeholder;
use crate::api::Bridge;
use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct MarkdownProps {
    pub markdown_text: AttrValue,
}

#[styled_component(Markdown)]
pub(crate) fn markdown(props: &MarkdownProps) -> HtmlResult {
    let set_error = use_atom_setter::<ErrorState>();

    let MarkdownProps { markdown_text } = props;

    let markdown_query = Bridge::use_query::<MarkdownQuery>(
        MarkdownQueryInput {
            value: markdown_text.to_string(),
        }
        .into(),
    )?;

    let children = match markdown_query.as_deref() {
        Ok(m) => m.value.to_html(),
        Err(_e) => {
            set_error(ErrorKind::Server.into());

            return Ok(html! {
                <>
                    <div class={css!("
                        margin-bottom: 10px;
                    ")}>
                        <Placeholder height="1rem" width="100%" />
                    </div>
                    <div class={css!("
                        margin-bottom: 10px;
                    ")}>
                        <Placeholder height="1rem" width="100%" />
                    </div>
                    <div class={css!("
                        margin-bottom: 30px;
                    ")}>
                        <Placeholder height="1rem" width="75%" />
                    </div>

                    <div class={css!("
                        margin-bottom: 10px;
                    ")}>
                        <Placeholder height="10rem" width="100%" />
                    </div>
                </>
            });
        }
    };

    Ok(html! {
        <div>{children}</div>
    })
}
