use async_trait::async_trait;
use worker::wasm_bindgen::UnwrapThrowExt;
use worker::{Fetch, Method, Request, RouteContext};

use crate::error::{Error, Result};
use crate::prelude::*;
use crate::RequestContext;
pub(crate) use messages::User;

#[async_trait(?Send)]
pub(crate) trait UserExt {
    async fn get(ctx: &RouteContext<RequestContext>, id: u64) -> Result<Option<User>>;
    async fn from_token(token: &str) -> Result<User>;
}

#[async_trait(?Send)]
impl UserExt for User {
    async fn get(ctx: &RouteContext<RequestContext>, id: u64) -> Result<Option<User>> {
        let user_store = ctx.kv("USERS")?;

        let value = user_store.get(&id.to_string()).await?;

        if let Some(m) = value {
            return Ok(Some(m.as_json::<User>()?));
        }

        let mut req = Request::new(
            &format!("https://api.github.com/user/{id}", id = id),
            Method::Get,
        )?;

        {
            let token = ctx.secret("GITHUB_API_KEY").unwrap_throw().to_string();
            req.headers_mut()?
                .set("authorization", &format!("Bearer {}", token))?;
        }

        let mut resp = Fetch::Request(req).send().await?;

        if resp.status_code() == 404 {
            return Ok(None);
        }

        let user = match resp.json::<User>().await {
            Ok(m) => m,
            Err(_) => return Err(Error::GitHub),
        };

        user_store
            .put(&user.id.to_string(), &user)?
            .expiration_ttl(24 * 60 * 60)
            .execute()
            .await?;

        Ok(Some(user))
    }

    async fn from_token(token: &str) -> Result<User> {
        let mut req = Request::new("https://api.github.com/user", Method::Get)?;
        {
            req.headers_mut()?
                .set("authorization", &format!("Bearer {}", token))?;
        }

        let mut resp = Fetch::Request(req).send().await?;

        match resp.json::<User>().await {
            Ok(m) => Ok(m),
            Err(_) => Err(Error::GitHub),
        }
    }
}
