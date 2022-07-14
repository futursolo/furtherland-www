use std::sync::Arc;

use fl_www_core::messages::PatchReplyInput;
use messages::ReplyInput;
use object_id::ObjectId;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use super::exts::FilterExt;
use crate::context::{RequestContext, ServerContext};
use crate::encoding::Encoding;
// use crate::db::residents as model;
use crate::error::HttpResult;
use crate::prelude::*;
use crate::reply::ReplyExt;

async fn get_replies(
    _lang: Language,
    _slug: String,
    _ctx: RequestContext,
) -> HttpResult<impl Reply> {
    Ok(warp::reply::html("not implemented"))
}

async fn get_reply(id: ObjectId, ctx: RequestContext) -> HttpResult<impl Reply> {
    let reply = messages::Reply::get(&ctx, id).await?;

    let resp = messages::Response::Success { content: reply };

    Ok(ctx.reply(&resp))
}

async fn post_reply(ctx: RequestContext, input: ReplyInput) -> HttpResult<impl Reply> {
    let reply = messages::Reply::create(&ctx, &input).await?;

    let resp = messages::Response::Success { content: reply };

    Ok(ctx.reply(&resp))
}

async fn patch_reply(
    _lang: Language,
    _slug: String,
    id: ObjectId,
    ctx: RequestContext,
    input: PatchReplyInput,
) -> HttpResult<impl Reply> {
    messages::Reply::patch(&ctx, id, &input).await?;

    let resp = messages::Response::Success { content: () };

    Ok(ctx.reply(&resp))
}

async fn delete_reply(
    _lang: Language,
    _slug: String,
    id: ObjectId,
    ctx: RequestContext,
) -> HttpResult<impl Reply> {
    messages::Reply::delete(&ctx, id).await?;

    let resp = messages::Response::Success { content: () };

    Ok(ctx.reply(&resp))
}

pub(crate) fn endpoints(ctx: Arc<ServerContext>) -> BoxedFilter<(impl Reply,)> {
    let get_replies = warp::path!("replies" / Language / String)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::get())
        .then(get_replies)
        .terminated();

    let post_reply = warp::path!("replies")
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::post())
        .and(Encoding::request_body_filter::<ReplyInput>())
        .then(post_reply)
        .terminated();

    let get_reply = warp::path!("replies" / ObjectId)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::get())
        .then(get_reply)
        .terminated();

    let patch_reply = warp::path!("replies" / Language / String / ObjectId)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::patch())
        .and(Encoding::request_body_filter::<PatchReplyInput>())
        .then(patch_reply)
        .terminated();

    let delete_reply = warp::path!("replies" / Language / String / ObjectId)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx))
        .and(warp::delete())
        .then(delete_reply)
        .terminated();

    get_replies
        .or(post_reply)
        .or(get_reply)
        .or(patch_reply)
        .or(delete_reply)
        .boxed()
}
