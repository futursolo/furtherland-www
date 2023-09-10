use stellation_bridge::links::LocalLink;
use stellation_bridge::registry::ResolverRegistry;
use stellation_bridge::Bridge as Bridge_;

pub use crate::routines::*;

mod metadata;
mod page;
mod template;
mod writing;

pub fn create_resolver_registry() -> ResolverRegistry<()> {
    ResolverRegistry::<()>::builder()
        .add_query::<ServerTimeQuery>()
        .add_mutation::<GreetingMutation>()
        .add_query::<MetadataQuery>()
        .add_query::<PageQuery>()
        .add_query::<WritingQuery>()
        .build()
}

pub type Link = LocalLink<()>;
pub type Bridge = Bridge_<Link>;