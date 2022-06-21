use async_trait::async_trait;
use chrono::Utc;
use messages::{Reply, ReplyInput, ResidencyStatus};
use object_id::ObjectId;
use sea_orm::entity::{ActiveModelTrait, EntityTrait, ModelTrait};
use sea_orm::Set;

use crate::context::RequestContext;
use crate::db;
use crate::db::replies as model;
use crate::error::{HttpError, HttpResult};
use crate::prelude::*;
use crate::resident::{Resident, ResidentExt};

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

    async fn get(ctx: &RequestContext, id: ObjectId) -> HttpResult<Self>
    where
        Self: Sized,
    {
        let reply_ent = match model::Entity::find_by_id(id.to_string())
            .one(ctx.db())
            .await?
        {
            Some(m) => m,
            None => return Err(HttpError::NotFound),
        };

        let resident_ent = reply_ent
            .find_related(db::residents::Entity)
            .one(ctx.db())
            .await?;

        let resident = match resident_ent {
            Some(m) => {
                Resident::get(
                    ctx,
                    m.github_id.try_into().map_err(|_| {
                        HttpError::DataIntegrity(format!(
                            "`{}` cannot be converted to u64",
                            m.github_id
                        ))
                    })?,
                )
                .await?
            }
            None => None,
        };

        Ok(Self {
            id: reply_ent.id.parse().map_err(|_| {
                HttpError::DataIntegrity(format!("`{}` is not a valid ObjectId.", reply_ent.id))
            })?,
            slug: reply_ent.slug,
            lang: reply_ent.lang,
            approved: Some(reply_ent.approved),
            resident,
            content: reply_ent.content,
            created_at: reply_ent.created_at,
        })
    }
}
