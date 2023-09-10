use async_trait::async_trait;
use stellation_bridge::resolvers::QueryResolver;
use stellation_bridge::routines::QueryResult;

use super::context::ResolverContext;
use crate::routines::PageQuery;

#[async_trait(?Send)]
impl QueryResolver for PageQuery {
    type Context = ResolverContext;

    async fn resolve(_ctx: &ResolverContext, _input: &Self::Input) -> QueryResult<Self> {
        todo!()
    }
}
