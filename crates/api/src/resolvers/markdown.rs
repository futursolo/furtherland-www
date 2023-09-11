use async_trait::async_trait;
use stellation_bridge::resolvers::QueryResolver;
use stellation_bridge::routines::QueryResult;

use super::context::ResolverContext;
use crate::markdown::parser::{HtmlCreator, Parser};
use crate::routines::MarkdownQuery;

#[async_trait(?Send)]
impl QueryResolver for MarkdownQuery {
    type Context = ResolverContext;

    async fn resolve(_ctx: &ResolverContext, input: &Self::Input) -> QueryResult<Self> {
        let root = HtmlCreator::new(Parser::new(&input.value)).into_root_node();

        Ok(Self { value: root }.into())
    }
}
