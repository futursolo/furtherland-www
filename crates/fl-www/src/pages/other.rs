use bounce::helmet::Helmet;
use components::{Main, SectionTitle};

use crate::prelude::*;

#[function_component(Other)]
pub(crate) fn other() -> Html {
    use_language();
    html! {
        <>
            <Helmet>
                <title>{fl!("not-found-title")}</title>
                <meta name="robots" content="noindex" />
            </Helmet>
            <Main>
                <SectionTitle>{fl!("not-found-title")}</SectionTitle>
                <div><p>{fl!("not-found-description")}</p></div>
            </Main>
        </>
    }
}
