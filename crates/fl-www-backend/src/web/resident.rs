use std::sync::Arc;

use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use super::exts::FilterExt;
use crate::context::{RequestContext, ServerContext};
use crate::error::{HttpError, HttpResult};
use crate::prelude::*;
use crate::resident::{Resident, ResidentExt};

async fn exchange_token(
    ctx: RequestContext,
    input: messages::AccessTokenInput,
) -> HttpResult<impl Reply> {
    #[derive(Serialize, Deserialize)]
    struct GitHubAccessTokenInput<'a, 'b, 'c> {
        client_id: &'a str,
        client_secret: &'b str,
        code: &'c str,
    }

    let body = GitHubAccessTokenInput {
        client_id: &ctx.github_client_id,
        client_secret: &ctx.github_client_secret,
        code: &input.code,
    };

    let access_token = ctx
        .http()
        .post("https://github.com/login/oauth/access_token")
        .json(&body)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .send()
        .and_then(|m| m.json::<messages::AccessToken>())
        .await
        .map_err(|_| HttpError::GitHub)?;

    let resp = messages::Response::Success {
        content: access_token,
    };

    Ok(warp::reply::json(&resp))
}

async fn get_myself(ctx: RequestContext) -> HttpResult<impl Reply> {
    let resident = match ctx.resident().cloned() {
        Some(m) => m,
        None => return Err(HttpError::Forbidden),
    };

    let resp = messages::Response::Success { content: resident };

    Ok(warp::reply::json(&resp))
}

async fn get_resident(github_id: u64, ctx: RequestContext) -> HttpResult<impl Reply> {
    let resident = match Resident::get(&ctx, github_id).await? {
        Some(ref m) => m.clone(),
        None => return Err(HttpError::NotFound),
    };

    let resp = messages::Response::Success { content: resident };

    Ok(warp::reply::json(&resp))
}

pub(crate) fn endpoints(ctx: Arc<ServerContext>) -> BoxedFilter<(impl Reply,)> {
    let post_access_token = warp::path!("residents" / "_oauth_access_token")
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::post())
        .and(warp::filters::body::json::<messages::AccessTokenInput>())
        .then(exchange_token)
        .terminated();

    let get_resident = warp::path!("residents" / u64)
        .and(warp::path::end())
        .and(RequestContext::filter(ctx.clone()))
        .and(warp::get())
        .then(get_resident)
        .terminated();

    let get_myself = warp::path!("residents" / "myself")
        .and(warp::path::end())
        .and(RequestContext::filter(ctx))
        .and(warp::get())
        .then(get_myself)
        .terminated();

    post_access_token.or(get_myself).or(get_resident).boxed()
}
