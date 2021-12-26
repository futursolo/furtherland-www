use async_trait::async_trait;
use worker::wasm_bindgen::UnwrapThrowExt;
use worker::{Fetch, Method, Request, RouteContext};

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

        let value = resident_store.get(&id.to_string()).await?;

        if let Some(m) = value.and_then(|m| m.as_json::<Resident>().ok()) {
            return Ok(Some(m));
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
            req.headers_mut()?
                .set("authorization", &format!("Bearer {}", token))?;
        }

        let mut resp = Fetch::Request(req).send().await?;

        match resp.json::<Resident>().await {
            Ok(m) => Ok(m),
            Err(_) => Err(Error::GitHub),
        }
    }
}
