use fl_www_backend as backend;
use stellation_bridge::links::LocalLink;
use stellation_bridge::registry::{ResolverRegistry, ResolverRegistryBuilder};
use stellation_bridge::Bridge as Bridge_;

use self::backend::db;
pub use self::context::ResolverContext;
use self::error::ResolverError;
pub use crate::routines::*;

mod context;
mod error;
mod highlight;
mod markdown;
mod metadata;
mod page;
mod replies;
mod resident;
mod writing;

pub fn create_resolver_registry() -> ResolverRegistry<ResolverContext> {
    ResolverRegistryBuilder::new()
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
