use crate::prelude::*;

use bounce::helmet::Helmet;

use super::Loading;
use components::{Author, AuthoringResident, Link, Main, SectionTitle};

#[function_component(Home)]
pub(crate) fn home() -> Html {
    let lang = use_language();

    let metadata = match use_metadata() {
        Some(m) => m,
        None => {
            return html! {
                <>
                    <Helmet>
                        <title>{fl!("home")}</title>
                    </Helmet>
                    <Loading />
                </>
            }
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

    html! {
        <>
            <Helmet>
                <title>{fl!("home")}</title>
            </Helmet>
            <Main>
                {writings}
            </Main>
        </>
    }
}
