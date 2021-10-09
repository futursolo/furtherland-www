use crate::prelude::*;

use yew_side_effect::title::Title;

use super::Other;
use components::{Comments, Main, Markdown, SectionTitle, WritingInfo};

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct WritingProps {
    pub slug: String,
}

#[function_component(Writing)]
pub(crate) fn writing(props: &WritingProps) -> Html {
    let lang = use_language();
    let metadata = use_metadata();

    metadata
        .writings()
        .iter()
        .rev()
        .filter(|m| m.lang == lang)
        .find(|m| m.slug == props.slug)
        .map(|m| {
            let title = m.get_title();
            html! {
                <>
                    <Title value={title.clone()} />
                    <Main>
                        <SectionTitle>{title}</SectionTitle>
                        <WritingInfo date={m.date} />
                        <Markdown markdown_text={m.get_content()} />
                        <Comments />
                    </Main>
                </>
            }
        })
        .unwrap_or_else(|| html! {<Other />})
}
