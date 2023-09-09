use async_trait::async_trait;
use stellation_bridge::resolvers::{MutationResolver, QueryResolver};
use stellation_bridge::routines::{MutationResult, QueryResult};
use time::OffsetDateTime;

pub use crate::routines::*;

#[async_trait(?Send)]
impl QueryResolver for ServerTimeQuery {
    type Context = ();

    async fn resolve(_ctx: &(), _input: &Self::Input) -> QueryResult<Self> {
        Ok(Self {
            value: OffsetDateTime::now_utc(),
        }
        .into())
    }
}

#[async_trait(?Send)]
impl MutationResolver for GreetingMutation {
    type Context = ();

    async fn resolve(_ctx: &(), name: &Self::Input) -> MutationResult<Self> {
        Ok(Self {
            message: format!("Hello, {name}!"),
        }
        .into())
    }
}
