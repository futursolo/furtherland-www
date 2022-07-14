use std::convert::Infallible;

use bounce::helmet::Helmet;
use bounce::prelude::*;
use components::{Main, Markdown, SectionTitle};
use yew_query::{use_query, Request, UseFetchHandle};

use super::{Loading, Other};
use crate::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct PageProps {
    pub slug: String,
}

#[styled_component(Page)]
pub(crate) fn page(props: &PageProps) -> Html {
    let lang = use_language();
    let set_error = use_atom_setter::<ErrorState>();

    let slug = props.slug.clone();
    let req: UseFetchHandle<String, Infallible> = use_query(move || {
        Request::builder()
            .url(format!(
                "/pages/{lang}/{slug}.md",
                lang = lang.as_str(),
                slug = slug,
            ))
            .build()
    });

    let full_content = match req.result() {
        None => return html! {<Loading />},
        Some(Err(e)) => {
            if let yew_query::Error::Response(ref e) = *e {
                if e.status() == 404 {
                    return html! {<Other />};
                }
            }

            set_error(ErrorKind::Server.into());

            return html! {<Loading />};
        }
        Some(Ok(m)) => {
            if !m
                .headers()
                .get("content-type")
                .ok()
                .map(|m| m.map(|m| m.contains("markdown")).unwrap_or(false))
                .unwrap_or(false)
            {
                return html! {<Other />};
            }

            m.data().trim().to_string()
        }
    };

    let (title, content) = full_content.split_once('\n').unwrap_or((&full_content, ""));
    let mut title = title.to_string();
    let content = content.trim().to_string();

    while title.starts_with('#') {
        title.remove(0);
    }
    let title = title.trim().to_string();

    html! {
        <>
            <Helmet>
                <title>{title.clone()}</title>
            </Helmet>
            <Main>
                <SectionTitle>{&title}</SectionTitle>
                <Markdown markdown_text={content} />
            </Main>
        </>
    }
}
