use serde::{Deserialize, Serialize};
use styling::{Colour, ThemeKind};
use yew_agent::{Agent, AgentLink, HandlerId, Public};

use crate::prelude::*;
use crate::types::Msg;

mod syntax;

pub async fn highlight(input: Request) -> Response {
    let Request::Highlight(i) = input;

    Response::Highlighted(HighlightOutput::new(i).await)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HighlightInput {
    pub content: String,
    pub language: String,
    pub theme_kind: ThemeKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HighlightOutput {
    pub fragments: Vec<(Colour, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Highlight(HighlightInput),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Highlighted(Option<HighlightOutput>),
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
        self.link
            .send_future(async move { Msg::Respond((highlight(msg).await, who)) })
    }

    fn name_of_resource() -> &'static str {
        "fl-agent-highlight.js"
    }
}
