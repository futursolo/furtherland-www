use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use worker::{Request, Response, RouteContext, Router};

use crate::error::{Error, Result};
use crate::prelude::*;
use crate::resident::{Resident, ResidentExt};
use crate::RequestContext;
use object_id::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ReplyValue {
    id: ObjectId,
    slug: String,
    approved: bool,
    resident_id: u64,
    content: String,
    lang: Language,
    created_at: DateTime<Utc>,
}

async fn post_reply(mut req: Request, ctx: RouteContext<RequestContext>) -> Result<Response> {
    let resident = match ctx.data().resident {
        Some(ref m) => m.clone(),
        None => return Err(Error::Forbidden),
    };

    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let lang = match ctx.param("lang").and_then(|m| m.parse::<Language>().ok()) {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let input = req
        .json::<messages::ReplyInput>()
        .await
        .map_err(|_e| Error::BadRequest)?;

    let reply_store = ctx.kv("REPLIES")?;
    let approved = resident.status() == messages::ResidencyStatus::Master;

    let reply = messages::Reply {
        id: ObjectId::new(),
        slug: slug.to_owned(),
        content: input.content.clone(),
        resident: Some(resident.clone()),
        approved: Some(approved),
        lang,
        created_at: Utc::now(),
    };

    let key = reply.key();

    let reply_value = ReplyValue {
        id: reply.id.clone(),
        slug: slug.to_owned(),
        approved: resident.status() == messages::ResidencyStatus::Master,
        resident_id: resident.id,
        content: input.content,
        lang,
        created_at: Utc::now(),
    };

    reply_store.put(&key, &reply_value)?.execute().await?;

    let resp = messages::Response::Success { content: reply };

    Ok(Response::from_json(&resp)?)
}

async fn get_replies(_req: Request, ctx: RouteContext<RequestContext>) -> Result<Response> {
    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let lang = match ctx.param("lang").and_then(|m| m.parse::<Language>().ok()) {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let reply_store = ctx.kv("REPLIES")?;

    // TODO: pagination.
    let reply_list = reply_store
        .list()
        .prefix(format!("{}:{}:", lang, slug))
        .execute()
        .await?;

    let mut replies = Vec::new();
    for key in reply_list.keys.iter() {
        let reply = match reply_store.get(&key.name).await? {
            Some(m) => m,
            None => continue,
        };

        let reply_value = reply.as_json::<ReplyValue>()?;

        let maybe_resident = Resident::get(&ctx, reply_value.resident_id).await?;

        let reply = messages::Reply {
            id: reply_value.id,
            slug: reply_value.slug,
            content: reply_value.content,
            resident: maybe_resident,
            approved: Some(reply_value.approved),
            lang: reply_value.lang,
            created_at: reply_value.created_at,
        };

        replies.push(reply);
    }

    let replies = messages::Replies {
        replies,
        cursor: reply_list.cursor,
    };

    let resp = messages::Response::Success { content: replies };

    Ok(Response::from_json(&resp)?)
}

async fn get_reply(_req: Request, ctx: RouteContext<RequestContext>) -> Result<Response> {
    let reply_store = ctx.kv("REPLIES")?;

    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let lang = match ctx.param("lang").and_then(|m| m.parse::<Language>().ok()) {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let reply_id = match ctx.param("id") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };
    let reply = match reply_store
        .get(&format!("{}:{}:{}", lang, slug, reply_id))
        .await?
    {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let reply_value = reply.as_json::<ReplyValue>()?;

    let maybe_resident = Resident::get(&ctx, reply_value.resident_id).await?;

    let reply = messages::Reply {
        id: reply_value.id,
        slug: reply_value.slug,
        content: reply_value.content,
        resident: maybe_resident,
        approved: Some(reply_value.approved),
        lang: reply_value.lang,
        created_at: reply_value.created_at,
    };

    let resp = messages::Response::Success { content: reply };

    Ok(Response::from_json(&resp)?)
}

/// Patches a Reply.
///
/// Note: This can take time until changes are reflected.
async fn patch_reply(mut req: Request, ctx: RouteContext<RequestContext>) -> Result<Response> {
    let resident = match ctx.data().resident {
        Some(ref m) => m.clone(),
        None => return Err(Error::Forbidden),
    };

    if resident.status() != messages::ResidencyStatus::Master {
        return Err(Error::Forbidden);
    }

    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let lang = match ctx.param("lang").and_then(|m| m.parse::<Language>().ok()) {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let reply_id = match ctx.param("id") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let input = req
        .json::<messages::PatchReplyInput>()
        .await
        .map_err(|_e| Error::BadRequest)?;

    let reply_store = ctx.kv("REPLIES")?;
    let reply_key = format!("{}:{}:{}", lang, slug, reply_id);

    let reply = match reply_store.get(&reply_key).await? {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let mut reply_value = reply.as_json::<ReplyValue>()?;

    if let Some(m) = input.approved {
        reply_value.approved = m;
    }

    if let Some(m) = input.content {
        reply_value.content = m;
    }

    reply_store.put(&reply_key, &reply_value)?.execute().await?;

    let resp = messages::Response::Success { content: () };

    Ok(Response::from_json(&resp)?)
}

async fn delete_reply(_req: Request, ctx: RouteContext<RequestContext>) -> Result<Response> {
    let resident = match ctx.data().resident {
        Some(ref m) => m.clone(),
        None => return Err(Error::Forbidden),
    };

    if resident.status() != messages::ResidencyStatus::Master {
        return Err(Error::Forbidden);
    }

    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let reply_id = match ctx.param("id") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let lang = match ctx.param("lang").and_then(|m| m.parse::<Language>().ok()) {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let reply_store = ctx.kv("REPLIES")?;

    let reply_key = format!("{}:{}:{}", lang, slug, reply_id);
    reply_store.delete(&reply_key).await?;

    let resp = messages::Response::Success { content: () };

    Ok(Response::from_json(&resp)?)
}

pub(crate) fn register_endpoints(router: Router<'_, RequestContext>) -> Router<'_, RequestContext> {
    router
        .post_async("/replies/:lang/:slug/", |m, n| async move {
            Ok(post_reply(m, n).await.unwrap_or_else(|e| e.into_response()))
        })
        .get_async("/replies/:lang/:slug/", |m, n| async move {
            Ok(get_replies(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .get_async("/replies/:lang/:slug/:id", |m, n| async move {
            Ok(get_reply(m, n).await.unwrap_or_else(|e| e.into_response()))
        })
        .patch_async("/replies/:lang/:slug/:id", |m, n| async move {
            Ok(patch_reply(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .delete_async("/replies/:lang/:slug/:id", |m, n| async move {
            Ok(delete_reply(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
}
