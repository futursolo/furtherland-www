use crate::prelude::*;

use yew_side_effect::title::Title;

use components::{Main, SectionTitle};
use store::AppDispatch;

pub(crate) struct BaseAbout {
    dispatch: AppDispatch,
}

impl Component for BaseAbout {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Title value=fl!("about") />
                <Main>
                    <SectionTitle>{fl!("about")}</SectionTitle>
                    <h1>{fl!("coming-soon")}</h1>
                </Main>
            </>
        }
    }
}

pub(crate) type About = WithDispatch<BaseAbout>;
