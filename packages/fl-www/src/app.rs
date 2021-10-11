use crate::pages::AppRouter;
use crate::prelude::*;

use components::{ErrorPopup, Footer, Header, Layout};

#[function_component(App)]
pub(crate) fn app() -> Html {
    html! {
        <Layout>
            <Header />
            <AppRouter />
            <Footer />
            <ErrorPopup />
        </Layout>
    }
}
