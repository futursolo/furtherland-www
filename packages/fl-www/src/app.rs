use crate::pages::AppRouter;
use crate::prelude::*;

// use crate::contexts::MetaLink;
use components::{ErrorPopup, Footer, Header, Layout};

#[function_component(App)]
pub(crate) fn app() -> Html {
    // let lang = use_language();

    // let feed_url = match lang {
    //     Language::Chinese => "/feed-zh.xml",
    //     Language::English => "/feed-en.xml",
    // };

    html! {
        <Layout>
            <Header />
            <AppRouter />
            <Footer />
            <ErrorPopup />
            // <MetaLink rel="alternate" href={feed_url} type_="application/atom+xml" />
        </Layout>
    }
}
