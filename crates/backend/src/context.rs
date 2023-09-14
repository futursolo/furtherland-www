use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context;
use octocrab::Octocrab;
use reqwest::Client;
use sea_orm::{Database, DatabaseConnection};

#[derive(Debug, Clone)]
pub struct BackendContext {
    pub github_client_id: String,
    pub github_client_secret: String,

    db: DatabaseConnection,

    http: Client,
    github: Octocrab,

    content_dir: PathBuf,
}

impl BackendContext {
    pub async fn from_env() -> anyhow::Result<Self> {
        let github_token = env::var("FL_WWW_GITHUB_TOKEN").context("no github token set")?;

        Ok(Self {
            github_client_id: env::var("FL_WWW_GITHUB_CLIENT_ID").context("no github token set")?,
            github_client_secret: env::var("FL_WWW_GITHUB_CLIENT_SECRET")
                .context("no github token set")?,

            db: Database::connect(
                env::var("FL_WWW_DATABASE_URL").context("no database url provided")?,
            )
            .await
            .context("failed to connect to database")?,

            http: Client::builder()
                .user_agent(format!("fl-www-server/{}", env!("CARGO_PKG_VERSION")))
                .timeout(Duration::from_secs(30))
                .build()
                .context("failed to create http client")?,
            github: Octocrab::builder()
                .personal_token(github_token)
                .build()
                .context("failed to create github client")?,
            content_dir: PathBuf::from(
                env::var("FL_WWW_BACKEND_CONTENT_DIR").context("no content dir")?,
            ),
        })
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub fn http(&self) -> &Client {
        &self.http
    }

    pub fn server_github(&self) -> &Octocrab {
        &self.github
    }

    pub fn github(&self) -> &Octocrab {
        self.server_github()
    }

    pub fn content_dir(&self) -> &Path {
        self.content_dir.as_path()
    }
}
