use pulldown_cmark::Parser;

use crate::prelude::*;
use misc::markdown::HtmlCreator;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct MarkdownProps {
    #[prop_or_default]
    pub dispatch: AppDispatch,
    pub markdown_text: String,
}

impl DispatchPropsMut for MarkdownProps {
    type Store = store::Store;

    fn dispatch(&mut self) -> &mut AppDispatch {
        &mut self.dispatch
    }
}

pub(crate) struct BaseMarkdown {
    props: MarkdownProps,
}

impl Component for BaseMarkdown {
    type Message = ();
    type Properties = MarkdownProps;

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
        let children = HtmlCreator::new(Parser::new(&self.props.markdown_text)).into_html();
        html! {
            <div class=self.style()>{children}</div>
        }
    }
}

impl YieldStyle for BaseMarkdown {
    fn style_str(&self) -> Cow<'static, str> {
        r#"
        "#
        .into()
    }
}

pub(crate) type Markdown = WithDispatch<BaseMarkdown>;
