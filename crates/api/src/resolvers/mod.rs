use stellation_bridge::links::LocalLink;
use stellation_bridge::registry::{ResolverRegistry, ResolverRegistryBuilder};
use stellation_bridge::Bridge as Bridge_;

pub use self::context::ResolverContext;
pub use crate::routines::*;

mod context;
mod error;
mod metadata;
mod page;
mod replies;
mod resident;
mod template;
mod writing;

pub fn create_resolver_registry() -> ResolverRegistry<ResolverContext> {
    ResolverRegistryBuilder::new()
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

pub type Link = LocalLink<ResolverContext>;
pub type Bridge = Bridge_<Link>;
