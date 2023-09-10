use async_trait::async_trait;
use stellation_bridge::resolvers::{MutationResolver, QueryResolver};
use stellation_bridge::routines::{QueryResult, MutationResult};

use crate::routines::RepliesQuery;
use crate::CreateReplyMutation;

#[async_trait(?Send)]
impl QueryResolver for RepliesQuery {
    type Context = ();

    async fn resolve(_ctx: &(), _input: &Self::Input) -> QueryResult<Self> {
        todo!()
    }
}

#[async_trait(?Send)]
impl MutationResolver for CreateReplyMutation {
    type Context = ();

    async fn resolve(_ctx: &(), _input: &Self::Input) -> MutationResult<Self> {
        todo!()
    }
}
