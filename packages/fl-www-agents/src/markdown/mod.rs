use serde::{Deserialize, Serialize};
use yew_agent::{Agent, AgentLink, HandlerId, Public};

use crate::prelude::*;

mod parser;
mod types;

pub use types::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Html(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Html(Root),
}

#[derive(Debug)]
pub enum Msg {
    Html((Root, HandlerId)),
}

pub struct Worker {
    link: AgentLink<Worker>,
}

impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) {
        let Msg::Html(m) = msg;

        self.link.respond(m.1, Response::Html(m.0));
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        use pulldown_cmark::Parser;

        use parser::HtmlCreator;

        let Request::Html(i) = msg;

        let root = HtmlCreator::new(Parser::new(&i)).into_root_node();

        self.link.send_message(Msg::Html((root, who)))
    }

    fn name_of_resource() -> &'static str {
        option_env!("FL_WORKER_MARKDOWN_PATH").unwrap_or("fl-worker-markdown.js")
    }
}
