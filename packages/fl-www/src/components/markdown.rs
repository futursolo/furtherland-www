use bounce::prelude::*;

use crate::prelude::*;
use agents::prelude::*;
use atoms::CacheState;
use fl_www_core::markdown::Root;
use misc::ToHtml;

use super::Placeholder;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct MarkdownProps {
    pub markdown_text: String,
}

#[styled_component(Markdown)]
pub(crate) fn markdown(props: &MarkdownProps) -> Html {
    let cache_state = use_slice::<CacheState>();

    let md_html = {
        let input = props.markdown_text.clone();
        let cache_state = cache_state.clone();
        use_state_eq(|| -> Option<Html> {
            cache_state.get::<String, Root>(&input).map(|m| m.to_html())
        })
    };

    let md_html_clone = md_html.clone();

    let worker = {
        let cache_state = cache_state.clone();
        let input = props.markdown_text.clone();
        use_bridge::<agents::markdown::Worker, _>(move |m| {
            if let agents::markdown::Response::Html(root) = m {
                let action =
                    CacheState::convert_action::<String, Root>(&input, root.clone()).unwrap_throw();
                cache_state.dispatch(action);
                md_html_clone.set(Some(root.to_html()));
            }
        })
    };
    {
        let md_html = md_html.clone();
        use_effect_with_deps(
            move |content| {
                let content = content.clone();

                if let Some(cached) = cache_state
                    .get::<String, Root>(&content)
                    .map(|m| m.to_html())
                {
                    md_html.set(Some(cached));
                } else {
                    worker.send(agents::markdown::Request::Html(content));
                }

                || {}
            },
            props.markdown_text.clone(),
        );
    }

    let children = match (*md_html).clone() {
        Some(m) => m,
        None => {
            return html! {
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
            }
        }
    };

    html! {
        <div>{children}</div>
    }
}
