use crate::prelude::*;

// use yew::agent::{Bridge, Bridged};

// use agents::highlight::Worker as HighlightWorker;
#[cfg(not(debug_assertions))]
use misc::highlight::{HighlightInput, HighlightOutput};

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct CodeBlockProps {
    #[prop_or_default]
    pub dispatch: AppDispatch,

    pub language: Option<String>,

    pub content: String,
}

impl_dispatch_mut!(CodeBlockProps);

pub(crate) enum CodeBlockMsg {
    #[cfg(not(debug_assertions))]
    Highlighted(Option<HighlightOutput>),
}

pub(crate) struct BaseCodeBlock {
    #[cfg(not(debug_assertions))]
    link: ComponentLink<Self>,

    // worker: Box<dyn Bridge<HighlightWorker>>,
    props: CodeBlockProps,

    hl_html: Option<Html>,
}

impl Component for BaseCodeBlock {
    type Message = CodeBlockMsg;
    type Properties = CodeBlockProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // let worker_cb = link.callback(|m| {
        //     let agents::highlight::Response::Highlighted(m) = m;
        //     CodeBlockMsg::Highlighted(m)
        // });

        Self {
            #[cfg(not(debug_assertions))]
            link: _link,
            // worker: HighlightWorker::bridge(worker_cb),
            props,
            hl_html: None,
        }
    }

    #[cfg(not(debug_assertions))]
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.highlight();
        }
    }

    #[cfg(not(debug_assertions))]
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let CodeBlockMsg::Highlighted(m) = msg;

        self.hl_html = m.map(|m| m.to_html());

        true
    }

    #[cfg(debug_assertions)]
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props.neq_assign(props);

        if changed {
            self.hl_html = None;

            #[cfg(not(debug_assertions))]
            self.highlight();
        }

        changed
    }

    fn view(&self) -> Html {
        // let children = self
        //     .code_with_highlight()
        //     .unwrap_or_else(|| self.props.content.as_str().into());
        //
        let children = self
            .hl_html
            .clone()
            .unwrap_or_else(|| self.props.content.as_str().into());

        html! {
            <pre class=self.style()>
                <code>
                    {children}
                </code>
            </pre>
        }
    }
}

impl BaseCodeBlock {
    #[cfg(not(debug_assertions))]
    fn highlight(&mut self) {
        let theme_kind = self.props.dispatch.state().theme.current_kind();
        let content = self.props.content.clone();
        let language = match self.props.language {
            Some(ref m) => m.to_string(),
            None => return,
        };
        self.link.send_future(async move {
            CodeBlockMsg::Highlighted(
                HighlightOutput::new(HighlightInput {
                    content,
                    language,
                    theme_kind,
                })
                .await,
            )
        });

        // self.worker
        //     .send(agents::highlight::Request::Highlight(HighlightInput {
        //         content,
        //         language,
        //         theme_kind,
        //     }))
    }
}

impl YieldStyle for BaseCodeBlock {
    fn style_str(&self) -> Cow<'static, str> {
        let theme = self.props.dispatch.state().theme.current();

        format!(
            r#"
                background-color: {};
                padding: 20px;
                box-sizing: border-box;
                border-radius: 3px;

                overflow-x: auto;
            "#,
            theme.colour.background.code,
        )
        .into()
    }
}

pub(crate) type CodeBlock = WithDispatch<BaseCodeBlock>;
