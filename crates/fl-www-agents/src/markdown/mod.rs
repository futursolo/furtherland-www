use fl_www_core::markdown::{HtmlCreator, Root};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use yew_agent::{HandlerId, Public, Worker as Agent, WorkerLink as AgentLink};

use crate::prelude::*;
use crate::types::Msg;

pub async fn markdown(input: Request) -> Response {
    use pulldown_cmark::Parser;

    match input {
        Request::Html(i) => {
            let root = HtmlCreator::new(Parser::new(&i)).into_root_node();

            Response::Html(root)
        }
        Request::Summary(i) => {
            let root = HtmlCreator::new(Parser::new(&i)).into_root_node();

            Response::Summary(root.to_text().graphemes(true).take(200).fold(
                String::with_capacity(200),
                |mut s, c| {
                    if c != " " || !s.ends_with(' ') {
                        s.push_str(c);
                    }

                    s
                },
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Request {
    Html(String),
    Summary(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Html(Root),
    Summary(String),
}

pub struct Worker {
    link: AgentLink<Worker>,
}

impl Agent for Worker {
    type Input = Request;
    type Message = Msg<Response>;
    type Output = Response;
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) {
        let Msg::Respond(m) = msg;

        self.link.respond(m.1, m.0);
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        let link = self.link.clone();
        spawn_local(async move { link.send_message(Msg::Respond((markdown(msg).await, who))) })
    }

    fn name_of_resource() -> &'static str {
        "fl-agent-markdown.js"
    }
}
