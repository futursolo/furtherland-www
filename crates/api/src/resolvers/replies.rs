use async_trait::async_trait;
use stellation_bridge::resolvers::{MutationResolver, QueryResolver};
use stellation_bridge::routines::{MutationResult, QueryResult};

use super::context::ResolverContext;
use crate::routines::RepliesQuery;
use crate::CreateReplyMutation;

#[async_trait(?Send)]
impl QueryResolver for RepliesQuery {
    type Context = ResolverContext;

    async fn resolve(_ctx: &ResolverContext, _input: &Self::Input) -> QueryResult<Self> {
        todo!()
    }
}

#[async_trait(?Send)]
impl MutationResolver for CreateReplyMutation {
    type Context = ResolverContext;

    async fn resolve(_ctx: &ResolverContext, _input: &Self::Input) -> MutationResult<Self> {
        todo!()
    }
}
