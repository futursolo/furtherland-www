use crate::prelude::*;

use yew_side_effect::title::Title;

use super::Other;
use components::{Comments, Main, Markdown, SectionTitle, WritingInfo};
use store::AppDispatch;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct WritingProps {
    #[prop_or_default]
    pub dispatch: AppDispatch,

    pub slug: String,
}

impl DispatchPropsMut for WritingProps {
    type Store = store::Store;

    fn dispatch(&mut self) -> &mut AppDispatch {
        &mut self.dispatch
    }
}

pub(crate) struct BaseWriting {
    props: WritingProps,
}

impl Component for BaseWriting {
    type Message = ();
    type Properties = WritingProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let lang = &self.props.dispatch.state().i18n.lang;
        let metadata = self.props.dispatch.state().metadata.current();

        metadata
            .writings()
            .iter()
            .rev()
            .filter(|m| &m.lang == lang)
            .find(|m| m.slug == self.props.slug)
            .map(|m| {
                let title = m.get_title();
                html! {
                    <>
                        <Title value=title.clone() />
                        <Main>
                            <SectionTitle>{title}</SectionTitle>
                            <WritingInfo date=m.date />
                            <Markdown markdown_text=m.get_content() />
                            <Comments />
                        </Main>
                    </>
                }
            })
            .unwrap_or_else(|| html! {<Other />})
    }
}

pub(crate) type Writing = WithDispatch<BaseWriting>;
