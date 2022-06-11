use async_trait::async_trait;
pub(crate) use messages::Resident;
use octocrab::Octocrab;
use reqwest::StatusCode;

use crate::context::ServerContext;
use crate::error::{HttpError, HttpResult};
use crate::prelude::*;

#[async_trait]
pub(crate) trait ResidentExt {
    async fn get(ctx: &ServerContext, id: u64) -> HttpResult<Option<Resident>>;
    async fn from_token(ctx: &ServerContext, token: &str) -> HttpResult<(Resident, Octocrab)>;
}

#[async_trait]
impl ResidentExt for Resident {
    // TODO: cache resident information to lower request number.
    async fn get(ctx: &ServerContext, id: u64) -> HttpResult<Option<Resident>> {
        let resp = ctx
            .http()
            .get(format!("https://api.github.com/user/{id}", id = id))
            .header("authorization", format!("Bearer {}", ctx.github_token))
            .header("accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|_| HttpError::GitHub)?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let resident = match resp.json::<Resident>().await {
            Ok(m) => m,
            Err(_) => return Err(HttpError::GitHub),
        };

        Ok(Some(resident))
    }

    async fn from_token(_ctx: &ServerContext, token: &str) -> HttpResult<(Resident, Octocrab)> {
        let github = Octocrab::builder()
            .personal_token(token.to_owned())
            .build()
            .expect("failed to create github client");

        let resident = github
            .get::<_, _, ()>("https://api.github.com/user", None)
            .await
            .map_err(|_| HttpError::GitHub)?;

        Ok((resident, github))
    }
}
