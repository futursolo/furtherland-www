use async_trait::async_trait;
use stellation_bridge::resolvers::QueryResolver;
use stellation_bridge::routines::QueryResult;

use crate::routines::MetadataQuery;

#[async_trait(?Send)]
impl QueryResolver for MetadataQuery {
    type Context = ();

    async fn resolve(_ctx: &(), _input: &Self::Input) -> QueryResult<Self> {
        todo!()
    }
}
