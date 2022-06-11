use std::env;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use warp::{Filter, Rejection};

use crate::error::HttpError;
use crate::resident::{Resident, ResidentExt};

#[derive(Debug, Clone)]
pub struct ServerContext {
    pub github_token: String,

    pub github_client_id: String,
    pub github_client_secret: String,

    http: Client,
}

impl ServerContext {
    pub fn from_env() -> Self {
        Self {
            github_token: env::var("GITHUB_TOKEN").expect("no github token set"),
            github_client_id: env::var("GITHUB_CLIENT_ID").expect("no github token set"),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET").expect("no github token set"),
            http: Client::builder()
                .user_agent(format!("fl-www-backend/{}", env!("CARGO_PKG_VERSION")))
                .timeout(Duration::from_secs(30))
                .build()
                .expect("failed to build client"),
        }
    }

    pub fn http(&self) -> &Client {
        &self.http
    }
}

#[derive(Debug, Clone)]
pub struct RequestContext {
    srv_ctx: Arc<ServerContext>,
    pub resident: Option<Resident>,
}

impl Deref for RequestContext {
    type Target = ServerContext;

    fn deref(&self) -> &Self::Target {
        &self.srv_ctx
    }
}

impl RequestContext {
    pub fn filter(
        ctx: Arc<ServerContext>,
    ) -> impl Filter<Extract = (RequestContext,), Error = Rejection> + Send + Sync + Clone + 'static
    {
        warp::header::optional::<String>("authorization").and_then(move |token: Option<String>| {
            let ctx = ctx.clone();

            async move {
                let resident = match token {
                    Some(m) => {
                        if !m.to_lowercase().starts_with("bearer ") {
                            return Err(Rejection::from(HttpError::Forbidden));
                        }

                        let token = m.chars().skip(7).collect::<String>();

                        Some(Resident::from_token(&ctx, &token).await?)
                    }

                    None => None,
                };

                Ok(RequestContext {
                    srv_ctx: ctx.clone(),
                    resident,
                })
            }
        })
    }
}
