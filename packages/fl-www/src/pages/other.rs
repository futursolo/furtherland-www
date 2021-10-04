use crate::prelude::*;

use yew_side_effect::title::Title;

use components::{Main, SectionTitle};
use store::AppDispatch;

pub(crate) struct BaseOther {
    dispatch: AppDispatch,
}

impl Component for BaseOther {
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
                <Title value=fl!("not-found-title") />
                <Main>
                    <SectionTitle>{fl!("not-found-title")}</SectionTitle>
                    <div><p>{fl!("not-found-description")}</p></div>
                </Main>
            </>
        }
    }
}

pub(crate) type Other = WithDispatch<BaseOther>;
