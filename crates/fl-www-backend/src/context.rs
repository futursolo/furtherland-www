use std::env;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use octocrab::Octocrab;
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
    github: Octocrab,
}

impl ServerContext {
    pub fn from_env() -> Self {
        let github_token = env::var("GITHUB_TOKEN").expect("no github token set");

        Self {
            github_token: github_token.clone(),
            github_client_id: env::var("GITHUB_CLIENT_ID").expect("no github token set"),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET").expect("no github token set"),
            http: Client::builder()
                .user_agent(format!("fl-www-backend/{}", env!("CARGO_PKG_VERSION")))
                .timeout(Duration::from_secs(30))
                .build()
                .expect("failed to create http client"),
            github: Octocrab::builder()
                .personal_token(github_token)
                .build()
                .expect("failed to create github client"),
        }
    }

    pub fn http(&self) -> &Client {
        &self.http
    }

    pub fn github(&self) -> &Octocrab {
        &self.github
    }

    pub fn server_github(&self) -> &Octocrab {
        &self.github
    }
}

#[derive(Debug, Clone)]
pub struct RequestContext {
    srv_ctx: Arc<ServerContext>,
    pub resident: Option<Resident>,
    pub resident_github: Option<Octocrab>,
}

impl Deref for RequestContext {
    type Target = ServerContext;

    fn deref(&self) -> &Self::Target {
        &self.srv_ctx
    }
}

impl RequestContext {
    pub fn resident_github(&self) -> Option<&Octocrab> {
        self.resident_github.as_ref()
    }

    pub fn github(&self) -> &Octocrab {
        self.resident_github()
            .unwrap_or_else(|| self.srv_ctx.github())
    }

    pub fn filter(
        ctx: Arc<ServerContext>,
    ) -> impl Filter<Extract = (RequestContext,), Error = Rejection> + Send + Sync + Clone + 'static
    {
        warp::header::optional::<String>("authorization").and_then(move |token: Option<String>| {
            let ctx = ctx.clone();

            async move {
                match token.map(|m| {
                    m.trim()
                        .to_lowercase()
                        .starts_with("bearer ")
                        .then(|| {
                            m.trim()
                                .chars()
                                .skip(7)
                                .collect::<String>()
                                .trim()
                                .to_owned()
                        })
                        .ok_or_else(|| Rejection::from(HttpError::Forbidden))
                }) {
                    Some(Ok(m)) => {
                        let (resident, github) = Resident::from_token(&ctx, &m).await?;

                        Ok(RequestContext {
                            srv_ctx: ctx.clone(),
                            resident: Some(resident),
                            resident_github: Some(
                                Octocrab::builder()
                                    .personal_token(m)
                                    .build()
                                    .expect("failed to create github client"),
                            ),
                        })
                    }
                    Some(Err(e)) => Err(e),

                    None => Ok(RequestContext {
                        srv_ctx: ctx.clone(),
                        resident: None,
                        resident_github: None,
                    }),
                }
            }
        })
    }
}
