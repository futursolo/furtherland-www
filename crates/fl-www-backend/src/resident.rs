use async_trait::async_trait;
pub(crate) use messages::Resident;
use octocrab::Octocrab;

use crate::context::RequestContext;
use crate::error::{HttpError, HttpResult};
use crate::prelude::*;

#[async_trait]
pub(crate) trait ResidentExt {
    async fn get(ctx: &RequestContext, id: u64) -> HttpResult<Option<Resident>>;
    async fn from_token(token: &str) -> HttpResult<(Resident, Octocrab)>;
}

#[async_trait]
impl ResidentExt for Resident {
    // TODO: cache resident information to lower request number.
    async fn get(ctx: &RequestContext, id: u64) -> HttpResult<Option<Resident>> {
        let resident = match ctx
            .github()
            .get::<_, _, ()>(format!("https://api.github.com/user/{id}", id = id), None)
            .await
        {
            Ok(m) => m,
            Err(e) => {
                if let octocrab::Error::GitHub { source: e, .. } = e {
                    if e.message == "Not Found" {
                        return Ok(None);
                    }
                }

                return Err(HttpError::GitHub);
            }
        };

        Ok(Some(resident))
    }

    async fn from_token(token: &str) -> HttpResult<(Resident, Octocrab)> {
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
