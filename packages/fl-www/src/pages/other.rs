use crate::prelude::*;

use yew_side_effect::title::Title;

use crate::contexts::Meta;
use components::{Main, SectionTitle};

#[function_component(Other)]
pub(crate) fn other() -> Html {
    use_language();
    html! {
        <>
            <Title value={fl!("not-found-title")} />
            <Meta name="robots" content="noindex" />
            <Main>
                <SectionTitle>{fl!("not-found-title")}</SectionTitle>
                <div><p>{fl!("not-found-description")}</p></div>
            </Main>
        </>
    }
}
