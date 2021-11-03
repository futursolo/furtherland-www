use crate::prelude::*;
use agents::prelude::*;
use misc::ToHtml;

use super::Placeholder;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct MarkdownProps {
    pub markdown_text: String,
}

#[styled_component(Markdown)]
pub(crate) fn markdown(props: &MarkdownProps) -> Html {
    let md_html = use_state_eq(|| -> Option<Html> { None });

    let md_html_clone = md_html.clone();

    let worker: UseBridgeHandle<agents::markdown::Worker> = use_bridge(move |m| {
        if let agents::markdown::Response::Html(root) = m {
            md_html_clone.set(Some(root.to_html()));
        }
    });

    use_effect_with_deps(
        move |content| {
            let content = content.clone();
            worker.send(agents::markdown::Request::Html(content));

            || {}
        },
        props.markdown_text.clone(),
    );

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
