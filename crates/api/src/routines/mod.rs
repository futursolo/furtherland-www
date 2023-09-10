use bounce::{Atom, Selector};
use serde::{Deserialize, Serialize};
use stellation_bridge::links::FetchLink;
use stellation_bridge::registry::RoutineRegistry;
use stellation_bridge::Bridge as Bridge_;
use thiserror::Error;

mod metadata;
mod page;
mod replies;
mod resident;
mod template;
mod writing;
pub use metadata::*;
pub use page::*;
pub use replies::*;
pub use resident::*;
pub use template::*;
pub use writing::*;

#[derive(Debug, Error, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Error {
    #[error("content not found")]
    NotFound,

    #[error("failed to communicate with server.")]
    Network,
}

pub fn create_routine_registry() -> RoutineRegistry {
    RoutineRegistry::builder()
        .add_query::<ServerTimeQuery>()
        .add_mutation::<GreetingMutation>()
        .add_query::<MetadataQuery>()
        .add_query::<PageQuery>()
        .add_query::<WritingQuery>()
        .add_query::<CurrentResidentQuery>()
        .add_mutation::<ExchangeTokenMutation>()
        .add_query::<RepliesQuery>()
        .add_mutation::<CreateReplyMutation>()
        .build()
}

pub type Link = FetchLink;
pub type Bridge = Bridge_<Link>;

pub fn create_frontend_bridge() -> Bridge {
    Bridge::new(Link::builder().routines(create_routine_registry()).build())
}

#[derive(Debug, PartialEq, Atom)]
pub struct FrontendBridge {
    inner: Bridge,
}

impl Default for FrontendBridge {
    fn default() -> Self {
        Self {
            inner: Bridge::new(Link::builder().routines(create_routine_registry()).build()),
        }
    }
}

impl AsRef<Bridge> for FrontendBridge {
    fn as_ref(&self) -> &Bridge {
        &self.inner
    }
}

impl Selector for FrontendBridge {
    fn select(states: &bounce::BounceStates) -> std::rc::Rc<Self> {
        states.get_atom_value()
    }
}
