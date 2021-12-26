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
    resident_id: u64,
    content: String,
}

pub(crate) async fn post_reply(
    mut req: Request,
    ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    let resident = match ctx.data().resident {
        Some(ref m) => m.clone(),
        None => return Err(Error::Forbidden),
    };

    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let input = req
        .json::<messages::ReplyInput>()
        .await
        .map_err(|_e| Error::BadRequest)?;

    let reply_store = ctx.kv("REPLIES")?;

    let id = ObjectId::new();
    let key = format!("{}:{}", slug, id);

    let reply = ReplyValue {
        id,
        slug: slug.to_owned(),
        resident_id: resident.id,
        content: input.content,
    };

    reply_store.put(&key, &reply)?.execute().await?;

    let resp = messages::Response::Success {
        content: messages::Reply {
            id: reply.id,
            slug: reply.slug,
            content: reply.content,
            resident: Some(resident),
        },
    };

    Ok(Response::from_json(&resp)?)
}

pub(crate) async fn get_replies(
    _req: Request,
    ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let reply_store = ctx.kv("REPLIES")?;

    // TODO: pagination.
    let reply_list = reply_store
        .list()
        .prefix(format!("{}:", slug))
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

pub(crate) async fn get_reply(
    _req: Request,
    ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    let reply_store = ctx.kv("REPLIES")?;

    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let reply_id = match ctx.param("id") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };
    let reply = match reply_store.get(&format!("{}:{}", slug, reply_id)).await? {
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
    };

    let resp = messages::Response::Success { content: reply };

    Ok(Response::from_json(&resp)?)
}

pub(crate) async fn patch_reply(
    _req: Request,
    _ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    todo!()
}

pub(crate) async fn delete_reply(
    _req: Request,
    _ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    todo!()
}

pub(crate) fn register_endpoints(router: Router<'_, RequestContext>) -> Router<'_, RequestContext> {
    router
        .post_async("/replies/:slug/", |m, n| async move {
            Ok(post_reply(m, n).await.unwrap_or_else(|e| e.into_response()))
        })
        .get_async("/replies/:slug/", |m, n| async move {
            Ok(get_replies(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .get_async("/replies/:slug/:id", |m, n| async move {
            Ok(get_reply(m, n).await.unwrap_or_else(|e| e.into_response()))
        })
        .patch_async("/replies/:slug/:id", |m, n| async move {
            Ok(patch_reply(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .delete_async("/replies/:slug/:id", |m, n| async move {
            Ok(delete_reply(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
}
