use crate::prelude::*;

use yew_side_effect::title::Title;

use components::{Main, SectionTitle};

#[function_component(About)]
pub(crate) fn about() -> Html {
    html! {
        <>
            <Title value={fl!("about")} />
            <Main>
                <SectionTitle>{fl!("about")}</SectionTitle>
                <h1>{fl!("coming-soon")}</h1>
            </Main>
        </>
    }
}
