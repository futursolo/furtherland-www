use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use worker::wasm_bindgen::JsValue;
use worker::wasm_bindgen::UnwrapThrowExt;
use worker::{Fetch, Method, Request, Response, RouteContext, Router};

use crate::error::{Error, Result};
use crate::prelude::*;
use crate::RequestContext;
pub(crate) use messages::Resident;

#[async_trait(?Send)]
pub(crate) trait ResidentExt {
    async fn get(ctx: &RouteContext<RequestContext>, id: u64) -> Result<Option<Resident>>;
    async fn from_token(token: &str) -> Result<Resident>;
}

#[async_trait(?Send)]
impl ResidentExt for Resident {
    async fn get(ctx: &RouteContext<RequestContext>, id: u64) -> Result<Option<Resident>> {
        let resident_store = ctx.kv("RESIDENTS")?;

        if let Some(m) = resident_store
            .get(&id.to_string())
            .json::<Resident>()
            .await?
        {
            return Ok(Some(m));
        }

        let mut req = Request::new(
            &format!("https://api.github.com/user/{id}", id = id),
            Method::Get,
        )?;

        {
            let token = ctx.secret("GITHUB_API_KEY").unwrap_throw().to_string();

            let headers = req.headers_mut()?;
            headers.set("authorization", &format!("Bearer {}", token))?;
            headers.set("accept", "application/vnd.github.v3+json")?;
            headers.set("user-agent", "fl-www-backend/0.0.1")?;
        }

        let mut resp = Fetch::Request(req).send().await?;

        if resp.status_code() == 404 {
            return Ok(None);
        }

        let resident = match resp.json::<Resident>().await {
            Ok(m) => m,
            Err(_) => return Err(Error::GitHub),
        };

        resident_store
            .put(&resident.id.to_string(), &resident)?
            .expiration_ttl(24 * 60 * 60)
            .execute()
            .await?;

        Ok(Some(resident))
    }

    async fn from_token(token: &str) -> Result<Resident> {
        let mut req = Request::new("https://api.github.com/user", Method::Get)?;
        {
            let headers = req.headers_mut()?;
            headers.set("authorization", &format!("Bearer {}", token))?;
            headers.set("accept", "application/vnd.github.v3+json")?;
            headers.set("user-agent", "fl-www-backend/0.0.1")?;
        }

        let mut resp = Fetch::Request(req).send().await?;

        match resp.json::<Resident>().await {
            Ok(m) => Ok(m),
            Err(_) => Err(Error::GitHub),
        }
    }
}

async fn exchange_token(mut req: Request, ctx: RouteContext<RequestContext>) -> Result<Response> {
    #[derive(Serialize, Deserialize)]
    struct GitHubAccessTokenInput {
        client_id: String,
        client_secret: String,
        code: String,
    }

    let input = req.json::<messages::AccessTokenInput>().await?;

    let body = GitHubAccessTokenInput {
        client_id: ctx.secret("GITHUB_CLIENT_ID").unwrap_throw().to_string(),
        client_secret: ctx
            .secret("GITHUB_CLIENT_SECRET")
            .unwrap_throw()
            .to_string(),
        code: input.code,
    };
    let body = JsValue::from_str(&serde_json::to_string(&body).unwrap_throw());
    let mut req = Request::new_with_init(
        "https://github.com/login/oauth/access_token",
        worker::RequestInit::new()
            .with_body(Some(body))
            .with_method(Method::Post),
    )?;
    {
        let headers = req.headers_mut()?;
        headers.set("content-type", "application/json")?;
        headers.set("accept", "application/json")?;
    }

    let mut resp = Fetch::Request(req).send().await?;
    let access_token = match resp.json::<messages::AccessToken>().await {
        Ok(m) => m,
        Err(_) => return Err(Error::GitHub),
    };

    let resp = messages::Response::Success {
        content: access_token,
    };

    Ok(Response::from_json(&resp)?)
}

async fn get_myself(_req: Request, ctx: RouteContext<RequestContext>) -> Result<Response> {
    let resident = match ctx.data.resident {
        Some(ref m) => m.clone(),
        None => return Err(Error::Forbidden),
    };

    let resp = messages::Response::Success { content: resident };

    Ok(Response::from_json(&resp)?)
}

pub(crate) fn register_endpoints(router: Router<'_, RequestContext>) -> Router<'_, RequestContext> {
    router
        .post_async("/residents/_oauth_access_token", |m, n| async move {
            Ok(exchange_token(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .options_async("/residents/_oauth_access_token", |m, n| async move {
            Ok(crate::options_cors(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .get_async("/residents/myself", |m, n| async move {
            Ok(get_myself(m, n).await.unwrap_or_else(|e| e.into_response()))
        })
}
