use async_trait::async_trait;
use chrono::Utc;
use messages::{Reply, ReplyInput, ResidencyStatus};
use object_id::ObjectId;
use sea_orm::entity::{ActiveModelTrait, EntityTrait};
use sea_orm::Set;

use crate::context::RequestContext;
use crate::db::replies as model;
use crate::error::{HttpError, HttpResult};
use crate::prelude::*;
use crate::resident::ResidentExt;

#[async_trait]
pub(crate) trait ReplyExt {
    type Entity: EntityTrait;

    async fn create_reply(ctx: &RequestContext, input: &ReplyInput) -> HttpResult<Self>
    where
        Self: Sized;

    async fn get(ctx: &RequestContext, id: ObjectId) -> HttpResult<Self>
    where
        Self: Sized;
}

#[async_trait]
impl ReplyExt for Reply {
    type Entity = model::Entity;

    async fn create_reply(ctx: &RequestContext, input: &ReplyInput) -> HttpResult<Self>
    where
        Self: Sized,
    {
        let resident = ctx.resident().cloned().ok_or(HttpError::Forbidden)?;
        let resident_ent = resident.to_entity(ctx).await?;

        let id = ObjectId::new();

        let approved = resident.status() == ResidencyStatus::Master;
        let created_at = Utc::now();

        model::ActiveModel {
            id: Set(id.to_string()),
            slug: Set(input.slug.clone()),
            lang: Set(input.lang),
            approved: Set(resident.status() == ResidencyStatus::Master),
            resident_id: Set(resident_ent.id),
            content: Set(input.content.clone()),
            created_at: Set(Utc::now()),
        }
        .save(ctx.db())
        .await?;

        Ok(Self {
            id,
            slug: input.slug.clone(),
            lang: input.lang,

            resident: Some(resident),
            content: input.content.clone(),

            approved: Some(approved),

            created_at,
        })
    }

    async fn get(_ctx: &RequestContext, _id: ObjectId) -> HttpResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
