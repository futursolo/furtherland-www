use std::cell::RefCell;
use std::convert::Infallible;
use std::rc::Rc;

use bounce::helmet::Helmet;
use bounce::prelude::*;
use components::{Author, AuthoringResident, Main, Markdown, Replies, SectionTitle};
use yew_agent::Bridged;
use yew_query::{use_pausable_query, Request, UseFetchHandle};

use super::{Loading, Other};
use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct WritingProps {
    pub slug: String,
}

#[function_component(Writing)]
pub(crate) fn writing(props: &WritingProps) -> Html {
    let lang = use_language();
    let metadata = use_metadata();
    let set_error = use_atom_setter::<ErrorState>();

    let writing_metadata = metadata.as_ref().and_then(|m| {
        m.writings()
            .iter()
            .rev()
            .filter(|m| m.lang == lang)
            .find(|m| m.slug == props.slug)
            .cloned()
    });

    let summary = use_state_eq(|| -> Option<String> { None });

    let summary_clone = summary.clone();
    let worker = use_state(move || {
        RefCell::new(agents::markdown::Worker::bridge(Rc::new(move |m| {
            if let agents::markdown::Response::Summary(s) = m {
                summary_clone.set(Some(s));
            }
        })))
    });

    // let base_url = use_base_url();

    let writing_metadata_clone = writing_metadata.clone();
    let req: UseFetchHandle<String, Infallible> = use_pausable_query(move || {
        let writing_metadata = writing_metadata_clone?;

        Some(
            Request::builder()
                .url(format!(
                    "/writings/{lang}/{date}/{slug}.md",
                    lang = writing_metadata.lang.as_str(),
                    date = writing_metadata.date.format("%Y-%m-%d"),
                    slug = writing_metadata.slug,
                ))
                .build(),
        )
    });

    let summary_clone = summary.clone();
    use_effect_with_deps(
        move |data| {
            summary_clone.set(None);
            if let Some(m) = data {
                worker
                    .borrow_mut()
                    .send(agents::markdown::Request::Summary(m.to_string()));
            }
            || {}
        },
        req.result().and_then(|m| m.ok()).map(|m| m.data()),
    );

    if metadata.is_none() {
        return html! {<Loading />};
    }

    let writing_metadata = match writing_metadata {
        Some(m) => m,
        None => return html! {<Other />},
    };

    let content = match req.result() {
        None => {
            return html! {
                <>
                    <Helmet>
                        <title>{&writing_metadata.title}</title>
                    </Helmet>
                    <Loading />
                </>
            }
        }
        Some(Err(e)) => {
            if let yew_query::Error::Response(ref e) = *e {
                if e.status() == 404 {
                    return html! {<Other />};
                }
            }

            set_error(ErrorKind::Server.into());

            return html! {
                <>
                    <Helmet>
                        <title>{&writing_metadata.title}</title>
                    </Helmet>
                    <Loading />
                </>
            };
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

            m.data()
                .split_once('\n')
                .map(|m| m.1)
                .unwrap_or("")
                .trim()
                .to_string()
        }
    };

    html! {
        <>
            <Helmet>
                <title>{&writing_metadata.title}</title>
            </Helmet>
            {if let Some(m) = (*summary).clone() {
                html! {
                    <Helmet>
                        <meta name="description" content={m} />
                    </Helmet>
                }
            } else {
                Html::default()
            }}
            <Main>
                <SectionTitle>{&writing_metadata.title}</SectionTitle>
                <Author author={AuthoringResident::Default} date={writing_metadata.date} />
                <Markdown markdown_text={content} />
                <Replies slug={props.slug.clone()} />
            </Main>
        </>
    }
}
