use async_trait::async_trait;
use stellation_bridge::resolvers::QueryResolver;
use stellation_bridge::routines::QueryResult;

use super::context::ResolverContext;
use crate::backend::metadata::MetadataExt;
use crate::core::prelude::Metadata;
use crate::routines::MetadataQuery;
use crate::RoutineError;

#[async_trait(?Send)]
impl QueryResolver for MetadataQuery {
    type Context = ResolverContext;

    async fn resolve(ctx: &ResolverContext, _input: &Self::Input) -> QueryResult<Self> {
        let metadata = Metadata::from_path(ctx.content_dir()).await.map_err(|e| {
            tracing::error!("failed to generate metadata, due to: {:?}", e);

            RoutineError::ServerOther
        })?;

        Ok(Self { value: metadata }.into())
    }
}
