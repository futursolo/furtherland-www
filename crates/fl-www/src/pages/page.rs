use bounce::helmet::Helmet;
use bounce::prelude::*;
use components::{Main, Markdown, SectionTitle};
use fl_www_api::{Bridge, PageQuery, PageQueryInput};

use super::{Loading, Other};
use crate::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct PageProps {
    pub slug: String,
}

#[styled_component(Page)]
pub(crate) fn page(props: &PageProps) -> HtmlResult {
    let lang = use_language();
    let set_error = use_atom_setter::<ErrorState>();

    let slug = props.slug.clone();
    let page_query = Bridge::use_query::<PageQuery>(
        PageQueryInput::builder()
            .lang(lang)
            .slug(slug)
            .build()
            .into(),
    )?;

    let full_content = match page_query.as_deref() {
        Err(e) => {
            if matches!(e, fl_www_api::Error::NotFound) {
                return Ok(html! {<Other />});
            }

            set_error(ErrorKind::Server.into());

            return Ok(html! {<Loading />});
        }
        Ok(m) => m.content.to_owned(),
    };

    let (title, content) = full_content.split_once('\n').unwrap_or((&full_content, ""));
    let mut title = title.to_string();
    let content = content.trim().to_string();

    while title.starts_with('#') {
        title.remove(0);
    }
    let title = title.trim().to_string();

    Ok(html! {
        <>
            <Helmet>
                <title>{title.clone()}</title>
            </Helmet>
            <Main>
                <SectionTitle>{&title}</SectionTitle>
                <Markdown markdown_text={content} />
            </Main>
        </>
    })
}
