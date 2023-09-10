use async_trait::async_trait;
use stellation_bridge::resolvers::{MutationResolver, QueryResolver};
use stellation_bridge::routines::{MutationResult, QueryResult};

use crate::routines::{CurrentResidentQuery, ExchangeTokenMutation};

#[async_trait(?Send)]
impl QueryResolver for CurrentResidentQuery {
    type Context = ();

    async fn resolve(_ctx: &(), _input: &Self::Input) -> QueryResult<Self> {
        todo!()
    }
}

#[async_trait(?Send)]
impl MutationResolver for ExchangeTokenMutation {
    type Context = ();

    async fn resolve(_ctx: &(), _input: &Self::Input) -> MutationResult<Self> {
        todo!()
    }
}
