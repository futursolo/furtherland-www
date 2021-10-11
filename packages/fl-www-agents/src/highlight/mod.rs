use serde::{Deserialize, Serialize};
use yew_agent::{Agent, AgentLink, HandlerId, Public};

use crate::prelude::*;
use styling::{Colour, ThemeKind};

mod syntax;

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

#[derive(Debug)]
pub enum Msg {
    Highlighted((Option<HighlightOutput>, HandlerId)),
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
        let Msg::Highlighted(m) = msg;

        self.link.respond(m.1, Response::Highlighted(m.0));
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        let Request::Highlight(i) = msg;

        self.link
            .send_future(async move { Msg::Highlighted((HighlightOutput::new(i).await, who)) })
    }

    fn name_of_resource() -> &'static str {
        option_env!("FL_WORKER_HIGHLIGHT_PATH").unwrap_or("fl-worker-highlight.js")
    }
}
