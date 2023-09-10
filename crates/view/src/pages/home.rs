use bounce::helmet::Helmet;
use components::{Author, AuthoringResident, Link, Main, SectionTitle};
use fl_www_api::{Bridge, MetadataQuery};

use super::Loading;
use crate::prelude::*;

#[function_component(Home)]
pub(crate) fn home() -> HtmlResult {
    let lang = use_language();

    let metadata_query = Bridge::use_query::<MetadataQuery>(().into())?;
    let metadata = match metadata_query.as_deref() {
        Ok(m) => m,
        Err(_) => {
            return Ok(html! {
                <>
                    <Helmet>
                        <title>{fl!("home")}</title>
                    </Helmet>
                    <Loading />
                </>
            })
        }
    };

    let writings = metadata
        .writings()
        .iter()
        .filter(|m| m.lang == lang)
        .map(|m| {
            html! {
                <>
                    <Link to={AppRoute::Writing{ slug: m.slug.clone(), lang }} styled={true}>
                        <SectionTitle font_size={2.0}>{m.title.to_owned()}</SectionTitle>
                    </Link>
                    <Author author={AuthoringResident::Default} date={m.date} />
                </>
            }
        })
        .collect::<Html>();

    Ok(html! {
        <>
            <Helmet>
                <title>{fl!("home")}</title>
            </Helmet>
            <Main>
                {writings}
            </Main>
        </>
    })
}
