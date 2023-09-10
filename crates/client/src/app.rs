use stellation_stylist::FrontendManagerProvider;
use yew::prelude::*;

use crate::view::Root;

#[function_component]
pub fn App() -> Html {
    html! {
        <Suspense fallback={Html::default()}>
            <FrontendManagerProvider>
                <Root />
            </FrontendManagerProvider>
        </Suspense>
    }
}
