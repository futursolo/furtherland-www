use crate::prelude::*;

use yew_side_effect::title::Title;

use super::Loading;
use components::{Link, Main, SectionTitle, WritingInfo};

#[function_component(Home)]
pub(crate) fn home() -> Html {
    let lang = use_language();

    let metadata = match use_metadata() {
        Some(m) => m,
        None => {
            return html! {
                <>
                    <Title value={fl!("home")} />
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
                    <WritingInfo date={m.date} />
                </>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <Title value={fl!("home")} />
            <Main>
                {writings}
            </Main>
        </>
    }
}
