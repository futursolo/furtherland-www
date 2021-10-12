use serde::{Deserialize, Serialize};
use yew_agent::{Agent, AgentLink, HandlerId, Public};

use crate::prelude::*;
use crate::types::Msg;

mod parser;
mod types;

pub use types::*;

pub async fn markdown(input: Request) -> Response {
    use parser::HtmlCreator;
    use pulldown_cmark::Parser;

    let Request::Html(i) = input;

    let root = HtmlCreator::new(Parser::new(&i)).into_root_node();

    Response::Html(root)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Html(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Html(Root),
}

pub struct Worker {
    link: AgentLink<Worker>,
}

impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = Msg<Response>;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) {
        let Msg::Respond(m) = msg;

        self.link.respond(m.1, m.0);
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        self.link
            .send_future(async move { Msg::Respond((markdown(msg).await, who)) })
    }

    fn name_of_resource() -> &'static str {
        option_env!("FL_WORKER_MARKDOWN_PATH").unwrap_or("fl-worker-markdown.js")
    }
}
