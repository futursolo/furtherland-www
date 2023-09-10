use async_trait::async_trait;
use chrono::Utc;
use fl_www_core::messages::Replies;
use fl_www_core::object_id::ObjectId;
use fl_www_core::prelude::Language;
use futures::stream::{BoxStream, StreamExt};
use futures::{stream, TryStreamExt};
use sea_orm::entity::{ActiveModelTrait, ActiveValue, EntityTrait, ModelTrait};
use sea_orm::{ColumnTrait, QueryFilter, QueryOrder, Set};
use stellation_bridge::resolvers::{MutationResolver, QueryResolver};
use stellation_bridge::routines::{MutationResult, QueryResult};

use super::context::ResolverContext;
use super::db::replies as model;
use super::error::{ResolverError, ResolverResult};
use super::resident::{Resident, ResidentExt};
use super::{db, RepliesQueryInput};
use crate::messages::{PatchReplyInput, Reply, ResidencyStatus};
use crate::routines::{CreateReplyInput, CreateReplyMutation, RepliesQuery};

#[async_trait]
pub(crate) trait ReplyExt {
    type Entity: EntityTrait;

    async fn create(ctx: &ResolverContext, input: &CreateReplyInput) -> ResolverResult<Self>
    where
        Self: Sized;
    async fn get(ctx: &ResolverContext, id: ObjectId) -> ResolverResult<Self>
    where
        Self: Sized;
    async fn delete(ctx: &ResolverContext, id: ObjectId) -> ResolverResult<()>;
    async fn patch(
        ctx: &ResolverContext,
        id: ObjectId,
        input: &PatchReplyInput,
    ) -> ResolverResult<()>;
    fn stream<'a>(
        ctx: &'a ResolverContext,
        lang: Language,
        slug: &'a str,
    ) -> BoxStream<'a, ResolverResult<Self>>
    where
        Self: Sized;
}

#[async_trait]
impl ReplyExt for Reply {
    type Entity = model::Entity;

    async fn create(ctx: &ResolverContext, input: &CreateReplyInput) -> ResolverResult<Self>
    where
        Self: Sized,
    {
        let resident = ctx
            .resident()
            .await?
            .cloned()
            .ok_or(ResolverError::Forbidden)?;
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

    async fn get(ctx: &ResolverContext, id: ObjectId) -> ResolverResult<Self>
    where
        Self: Sized,
    {
        let reply_ent = match model::Entity::find_by_id(id.to_string())
            .one(ctx.db())
            .await?
        {
            Some(m) => m,
            None => return Err(ResolverError::NotFound),
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
                        ResolverError::DataIntegrity(format!(
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
                ResolverError::DataIntegrity(format!("`{}` is not a valid ObjectId.", reply_ent.id))
            })?,
            slug: reply_ent.slug,
            lang: reply_ent.lang,
            approved: Some(reply_ent.approved),
            resident,
            content: reply_ent.content,
            created_at: reply_ent.created_at,
        })
    }

    async fn delete(ctx: &ResolverContext, id: ObjectId) -> ResolverResult<()> {
        let resident = ctx
            .resident()
            .await?
            .cloned()
            .ok_or(ResolverError::Forbidden)?;
        if resident.status() != ResidencyStatus::Master {
            return Err(ResolverError::Forbidden);
        }

        let reply_ent = match model::Entity::find_by_id(id.to_string())
            .one(ctx.db())
            .await?
        {
            Some(m) => m,
            None => return Err(ResolverError::NotFound),
        };

        reply_ent.delete(ctx.db()).await?;

        Ok(())
    }

    async fn patch(
        ctx: &ResolverContext,
        id: ObjectId,
        input: &PatchReplyInput,
    ) -> ResolverResult<()> {
        let resident = ctx
            .resident()
            .await?
            .cloned()
            .ok_or(ResolverError::Forbidden)?;
        if resident.status() != ResidencyStatus::Master {
            return Err(ResolverError::Forbidden);
        }

        let reply_ent = match model::Entity::find_by_id(id.to_string())
            .one(ctx.db())
            .await?
        {
            Some(m) => m,
            None => return Err(ResolverError::NotFound),
        };

        model::ActiveModel {
            id: ActiveValue::Unchanged(reply_ent.id),
            approved: match input.approved {
                Some(m) => ActiveValue::Set(m),
                None => ActiveValue::NotSet,
            },
            content: match input.content {
                Some(ref m) => ActiveValue::Set(m.clone()),
                None => ActiveValue::NotSet,
            },
            ..Default::default()
        }
        .save(ctx.db())
        .await?;

        Ok(())
    }

    fn stream<'a>(
        ctx: &'a ResolverContext,
        lang: Language,
        slug: &'a str,
    ) -> BoxStream<'a, ResolverResult<Self>>
    where
        Self: Sized,
    {
        stream::once(async move {
            let s = model::Entity::find()
                .filter(model::Column::Slug.eq(slug))
                .filter(model::Column::Lang.eq(lang))
                .order_by_asc(model::Column::Id)
                // .skip(50) This is for cursor, reserved when someday there are more than 50
                // replies on a single article.
                .stream(ctx.db())
                .await?
                .map_err(ResolverError::from)
                .and_then(move |reply_ent| async move {
                    let resident_ent = reply_ent
                        .find_related(db::residents::Entity)
                        .one(ctx.db())
                        .await?;

                    let resident = match resident_ent {
                        Some(m) => {
                            Resident::get(
                                ctx,
                                m.github_id.try_into().map_err(|_| {
                                    ResolverError::DataIntegrity(format!(
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
                            ResolverError::DataIntegrity(format!(
                                "`{}` is not a valid ObjectId.",
                                reply_ent.id
                            ))
                        })?,
                        slug: reply_ent.slug,
                        lang: reply_ent.lang,
                        approved: Some(reply_ent.approved),
                        resident,
                        content: reply_ent.content,
                        created_at: reply_ent.created_at,
                    })
                })
                .take(50)
                .boxed();

            ResolverResult::Ok(s)
        })
        .try_flatten()
        .boxed()
    }
}

#[async_trait(?Send)]
impl QueryResolver for RepliesQuery {
    type Context = ResolverContext;

    async fn resolve(ctx: &ResolverContext, input: &Self::Input) -> QueryResult<Self> {
        let RepliesQueryInput { lang, ref slug } = input;

        let replies = Reply::stream(ctx, *lang, slug);

        let replies = Replies {
            replies: replies.try_collect().await?,
        };

        Ok(Self { content: replies }.into())
    }
}

#[async_trait(?Send)]
impl MutationResolver for CreateReplyMutation {
    type Context = ResolverContext;

    async fn resolve(ctx: &ResolverContext, input: &Self::Input) -> MutationResult<Self> {
        let reply = Reply::create(ctx, input).await?;

        Ok(Self { content: reply }.into())
    }
}

// async fn get_replies(lang: Language, slug: String, ctx: RequestContext) -> HttpResult<impl Reply>
// {     let replies = messages::Reply::stream(&ctx, lang, &slug);
//     let replies = messages::Replies {
//         replies: replies.try_collect().await?,
//     };

//     let resp = messages::Response::Success { content: replies };

//     Ok(ctx.reply(&resp))
// }

// async fn get_reply(id: ObjectId, ctx: RequestContext) -> HttpResult<impl Reply> {
//     let reply = messages::Reply::get(&ctx, id).await?;

//     let resp = messages::Response::Success { content: reply };

//     Ok(ctx.reply(&resp))
// }

// async fn post_reply(ctx: RequestContext, input: ReplyInput) -> HttpResult<impl Reply> {
//     let reply = messages::Reply::create(&ctx, &input).await?;

//     let resp = messages::Response::Success { content: reply };

//     Ok(ctx.reply(&resp))
// }

// async fn patch_reply(
//     _lang: Language,
//     _slug: String,
//     id: ObjectId,
//     ctx: RequestContext,
//     input: PatchReplyInput,
// ) -> HttpResult<impl Reply> { messages::Reply::patch(&ctx, id, &input).await?;

//     let resp = messages::Response::Success { content: () };

//     Ok(ctx.reply(&resp))
// }

// async fn delete_reply(
//     _lang: Language,
//     _slug: String,
//     id: ObjectId,
//     ctx: RequestContext,
// ) -> HttpResult<impl Reply> { messages::Reply::delete(&ctx, id).await?;

//     let resp = messages::Response::Success { content: () };

//     Ok(ctx.reply(&resp))
// }
