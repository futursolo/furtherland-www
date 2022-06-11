use std::sync::Arc;

use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::context::{RequestContext, ServerContext};
use crate::error::{HttpError, HttpResult};
use crate::prelude::*;

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
    let resident = match ctx.resident {
        Some(ref m) => m.clone(),
        None => return Err(HttpError::Forbidden),
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
        .and_then(|m, i| exchange_token(m, i).map_err(warp::reject::custom));

    let get_myself = warp::path!("residents" / "myself")
        .and(warp::path::end())
        .and(RequestContext::filter(ctx))
        .and(warp::get())
        .and_then(|m| get_myself(m).map_err(warp::reject::custom));

    post_access_token.or(get_myself).boxed()
}
