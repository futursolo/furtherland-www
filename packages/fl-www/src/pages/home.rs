use crate::prelude::*;

use yew_side_effect::title::Title;

use components::{Link, Main, SectionTitle, WritingInfo};
use store::AppDispatch;

pub(crate) struct BaseHome {
    dispatch: AppDispatch,
}

impl Component for BaseHome {
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
        let lang = &self.dispatch.state().i18n.lang;
        let metadata = self.dispatch.state().metadata.current();

        let writings = metadata
            .writings()
            .iter()
            .rev()
            .filter(|m| &m.lang == lang)
            .map(|m| {
                html! {
                    <>
                        <Link to=lang.route_i18n(AppRoute::Writing(m.slug.clone())) styled=true>
                            <SectionTitle font_size=2.0>{m.get_title()}</SectionTitle>
                        </Link>
                        <WritingInfo date=m.date />
                    </>
                }
            })
            .collect::<Html>();

        html! {
            <>
                <Title value=fl!("home") />
                <Main>
                    {writings}
                </Main>
            </>
        }
    }
}

pub(crate) type Home = WithDispatch<BaseHome>;
