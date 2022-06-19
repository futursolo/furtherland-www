use std::sync::Arc;

use object_id::ObjectId;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use super::exts::FilterExt;
use crate::context::{RequestContext, ServerContext};
// use crate::db::residents as model;
use crate::error::{HttpError, HttpResult};
use crate::prelude::*;
use crate::resident::ResidentExt;

async fn get_replies(
    _lang: Language,
    _slug: String,
    _ctx: RequestContext,
) -> HttpResult<impl Reply> {
    Ok(warp::reply::html("not implemented"))
}

async fn get_reply(
    _lang: Language,
    _slug: String,
    _id: ObjectId,
    _ctx: RequestContext,
) -> HttpResult<impl Reply> {
    Ok(warp::reply::html("not implemented"))
}

async fn post_reply(
    _lang: Language,
    _slug: String,
    _id: ObjectId,
    ctx: RequestContext,
) -> HttpResult<impl Reply> {
    let resident = ctx.resident().ok_or(HttpError::Forbidden)?;
    let _resident_ent = resident.to_entity(&ctx).await?;

    Ok(warp::reply::html("not implemented"))
}

async fn patch_reply(
    _lang: Language,
    _slug: String,
    _id: ObjectId,
    _ctx: RequestContext,
) -> HttpResult<impl Reply> {
    Ok(warp::reply::html("not implemented"))
}

async fn delete_reply(
    _lang: Language,
    _slug: String,
    _id: ObjectId,
    _ctx: RequestContext,
) -> HttpResult<impl Reply> {
    Ok(warp::reply::html("not implemented"))
}

pub(crate) fn endpoints(ctx: Arc<ServerContext>) -> BoxedFilter<(impl Reply,)> {
    let get_replies = warp::path!("replies" / Language / String)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::get())
        .then(get_replies)
        .terminated();

    let get_reply = warp::path!("replies" / Language / String / ObjectId)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::get())
        .then(get_reply)
        .terminated();

    let post_reply = warp::path!("replies" / Language / String / ObjectId)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::post())
        .then(post_reply)
        .terminated();

    let patch_reply = warp::path!("replies" / Language / String / ObjectId)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::patch())
        .then(patch_reply)
        .terminated();

    let delete_reply = warp::path!("replies" / Language / String / ObjectId)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx))
        .and(warp::delete())
        .then(delete_reply)
        .terminated();

    get_replies
        .or(get_reply)
        .or(post_reply)
        .or(patch_reply)
        .or(delete_reply)
        .boxed()
}
