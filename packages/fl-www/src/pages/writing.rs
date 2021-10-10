use std::convert::Infallible;

use crate::prelude::*;

use yew_side_effect::title::Title;

use super::{Loading, Other};
use client::{use_pausable_request, ClientError, UseFetchHandle};
use components::{Comments, Main, Markdown, SectionTitle, WritingInfo};
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, Request, StatusCode};
use utils::get_base_url;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct WritingProps {
    pub slug: String,
}

#[function_component(Writing)]
pub(crate) fn writing(props: &WritingProps) -> Html {
    let lang = use_language();
    let metadata = use_metadata();
    let error = use_error_state();

    let writing_metadata = metadata.as_ref().and_then(|m| {
        m.writings()
            .iter()
            .rev()
            .filter(|m| m.lang == lang)
            .find(|m| m.slug == props.slug)
            .cloned()
    });

    let writing_metadata_clone = writing_metadata.clone();
    let req: UseFetchHandle<String, Infallible> = use_pausable_request(move || {
        let writing_metadata = writing_metadata_clone?;

        let mut url = get_base_url()?;

        let path = format!(
            "/writings/{lang}/{date}/{slug}.md",
            lang = writing_metadata.lang.as_str(),
            date = writing_metadata.date.format("%Y-%m-%d"),
            slug = writing_metadata.slug,
        );

        url.set_path(&path);

        Some(Request::new(Method::GET, url))
    });

    if metadata.is_none() {
        return html! {<Loading />};
    }

    let writing_metadata = match writing_metadata {
        Some(m) => m,
        None => return html! {<Other />},
    };

    let content = match req {
        UseFetchHandle::Loading => {
            return html! {
                <>
                    <Title value={writing_metadata.title} />
                    <Loading />
                </>
            }
        }
        UseFetchHandle::Err(e) => {
            if let ClientError::Reqwest(ref e) = *e {
                if e.status() == Some(StatusCode::NOT_FOUND) {
                    return html! {<Other />};
                }
            }

            error.set(ErrorKind::Server);

            return html! {
                <>
                    <Title value={writing_metadata.title} />
                    <Loading />
                </>
            };
        }
        UseFetchHandle::Ok(m) => {
            if !m
                .headers()
                .get(CONTENT_TYPE)
                .map(|m| m.to_str().map(|m| m.contains("markdown")).unwrap_or(false))
                .unwrap_or(false)
            {
                return html! {<Other />};
            }

            m.data()
                .split_once("\n")
                .map(|m| m.1)
                .unwrap_or("")
                .trim()
                .to_string()
        }
    };

    html! {
        <>
            <Title value={writing_metadata.title.clone()} />
            <Main>
                <SectionTitle>{&writing_metadata.title}</SectionTitle>
                <WritingInfo date={writing_metadata.date} />
                <Markdown markdown_text={content} />
                <Comments />
            </Main>
        </>
    }
}
