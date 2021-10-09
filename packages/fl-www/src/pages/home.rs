use crate::prelude::*;

use yew_side_effect::title::Title;

use components::{Link, Main, SectionTitle, WritingInfo};

#[function_component(Home)]
pub(crate) fn home() -> Html {
    let lang = use_language();
    let metadata = use_metadata();

    let writings = metadata
        .writings()
        .iter()
        .rev()
        .filter(|m| m.lang == lang)
        .map(|m| {
            html! {
                <>
                    <Link to={AppRoute::Writing{ slug: m.slug.clone(), lang }} styled={true}>
                        <SectionTitle font_size={2.0}>{m.get_title()}</SectionTitle>
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
