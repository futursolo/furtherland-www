use async_trait::async_trait;
use stellation_bridge::resolvers::{MutationResolver, QueryResolver};
use stellation_bridge::routines::{MutationResult, QueryResult};
use time::OffsetDateTime;

use super::context::ResolverContext;
pub use crate::routines::*;

#[async_trait(?Send)]
impl QueryResolver for ServerTimeQuery {
    type Context = ResolverContext;

    async fn resolve(_ctx: &ResolverContext, _input: &Self::Input) -> QueryResult<Self> {
        Ok(Self {
            value: OffsetDateTime::now_utc(),
        }
        .into())
    }
}

#[async_trait(?Send)]
impl MutationResolver for GreetingMutation {
    type Context = ResolverContext;

    async fn resolve(_ctx: &ResolverContext, name: &Self::Input) -> MutationResult<Self> {
        Ok(Self {
            message: format!("Hello, {name}!"),
        }
        .into())
    }
}
