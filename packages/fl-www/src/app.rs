use crate::pages::AppRouter;
use crate::prelude::*;

use components::{Footer, Header, Layout};

pub(crate) struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Layout>
                <Header />
                <AppRouter />
                <Footer />
            </Layout>
        }
    }
}
