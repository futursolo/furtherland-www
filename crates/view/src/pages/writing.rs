use bounce::helmet::Helmet;
use bounce::prelude::*;
use components::{Author, AuthoringResident, Main, Markdown, Replies, SectionTitle};
use fl_www_api::{Bridge, WritingQuery, WritingQueryInput};

use super::{Loading, Other};
use crate::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct WritingProps {
    pub slug: String,
}

#[function_component(Writing)]
pub(crate) fn writing(props: &WritingProps) -> HtmlResult {
    let lang = use_language();

    let set_error = use_atom_setter::<ErrorState>();

    let writing_query = Bridge::use_query::<WritingQuery>(
        WritingQueryInput::builder()
            .lang(lang)
            .slug(props.slug.clone())
            .build()
            .into(),
    )?;

    let writing = match writing_query.as_deref() {
        Err(e) => {
            if matches!(e, fl_www_api::RoutineError::NotFound) {
                return Ok(html! {<Other />});
            }
            set_error(ErrorKind::Server.into());

            return Ok(html! { <Loading /> });
        }
        Ok(m) => m.clone(),
    };

    Ok(html! {
        <>
            <Helmet>
                <title>{&writing.title}</title>
            </Helmet>
            {if let Some(m) = writing.summary.clone() {
                html! {
                    <Helmet>
                        <meta name="description" content={m} />
                    </Helmet>
                }
            } else {
                Html::default()
            }}
            <Main>
                <SectionTitle>{&writing.title}</SectionTitle>
                <Author author={AuthoringResident::Default} date={writing.date} />
                <Markdown markdown_text={writing.content.to_owned()} />
                <Replies slug={props.slug.clone()} />
            </Main>
        </>
    })
}
