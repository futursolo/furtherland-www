use async_trait::async_trait;
use stellation_bridge::resolvers::QueryResolver;
use stellation_bridge::routines::QueryResult;

use crate::routines::WritingQuery;

#[async_trait(?Send)]
impl QueryResolver for WritingQuery {
    type Context = ();

    async fn resolve(_ctx: &(), _input: &Self::Input) -> QueryResult<Self> {
        // TODO: Content...
        // m.data()
        // .split_once('\n')
        // .map(|m| m.1)
        // .unwrap_or("")
        // .trim()
        // .to_string()

        todo!()
    }
}
