use crate::prelude::*;

use yew_side_effect::title::Title;

use components::{Main, SectionTitle};

#[function_component(Other)]
pub(crate) fn other() -> Html {
    html! {
        <>
            // <Title value={fl!("not-found-title")} />
            <Main>
                <SectionTitle>{fl!("not-found-title")}</SectionTitle>
                <div><p>{fl!("not-found-description")}</p></div>
            </Main>
        </>
    }
}
