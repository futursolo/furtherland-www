use async_trait::async_trait;
use sea_orm::entity::EntityTrait;

use crate::context::RequestContext;
use crate::db::replies as model;
use crate::error::{HttpError, HttpResult};
use crate::prelude::messages::{Reply, ReplyInput};
use crate::resident::ResidentExt;

#[async_trait]
pub(crate) trait ReplyExt {
    type Entity: EntityTrait;

    async fn create_reply(ctx: &RequestContext, input: &ReplyInput) -> HttpResult<Self>
    where
        Self: Sized;
}

#[async_trait]
impl ReplyExt for Reply {
    type Entity = model::Entity;

    async fn create_reply(ctx: &RequestContext, _input: &ReplyInput) -> HttpResult<Self>
    where
        Self: Sized,
    {
        let resident = ctx.resident().ok_or(HttpError::Forbidden)?;
        let _resident_ent = resident.to_entity(ctx).await?;

        todo!()
    }
}
