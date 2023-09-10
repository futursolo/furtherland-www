use bounce::{Atom, Selector};
use stellation_bridge::links::FetchLink;
use stellation_bridge::registry::RoutineRegistry;
use stellation_bridge::Bridge as Bridge_;

mod error;
mod highlight;
pub mod markdown;
mod metadata;
mod page;
mod replies;
mod resident;
mod writing;
pub use error::RoutineError;
pub use highlight::*;
pub use markdown::*;
pub use metadata::*;
pub use page::*;
pub use replies::*;
pub use resident::*;
pub use writing::*;

pub fn create_routine_registry() -> RoutineRegistry {
    RoutineRegistry::builder()
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
