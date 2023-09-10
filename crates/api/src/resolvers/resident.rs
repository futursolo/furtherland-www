use async_trait::async_trait;
use chrono::Utc;
use futures::future::TryFutureExt;
pub(crate) use messages::Resident;
use octocrab::Octocrab;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use stellation_bridge::resolvers::{MutationResolver, QueryResolver};
use stellation_bridge::routines::{MutationResult, QueryResult};

use super::backend::BackendContext;
use super::context::ResolverContext;
use super::db::residents as model;
use super::error::ResolverResult;
use super::ResolverError;
use crate::routines::{CurrentResidentQuery, ExchangeTokenMutation};
use crate::{messages, ResidentQuery, ResidentQueryInput, RoutineError};

#[async_trait]
pub(crate) trait ResidentExt {
    type Entity: EntityTrait;

    async fn get(ctx: &ResolverContext, id: u64) -> ResolverResult<Option<Self>>
    where
        Self: Sized;
    async fn from_token(ctx: &BackendContext, token: &str) -> ResolverResult<(Resident, Octocrab)>;
    async fn to_entity(
        &self,
        ctx: &ResolverContext,
    ) -> ResolverResult<<Self::Entity as EntityTrait>::Model>;
}

#[async_trait]
impl ResidentExt for Resident {
    type Entity = model::Entity;

    async fn to_entity(
        &self,
        ctx: &ResolverContext,
    ) -> ResolverResult<<Self::Entity as EntityTrait>::Model> {
        let ent = model::Entity::find()
            .filter(model::Column::GithubId.eq(self.id))
            .one(ctx.db())
            .await?;

        ent.ok_or_else(|| {
            ResolverError::DataIntegrity(format!(
                "Cannot find corresponding entity for resident: {}",
                self.id
            ))
        })
    }

    async fn get(ctx: &ResolverContext, id: u64) -> ResolverResult<Option<Self>>
    where
        Self: Sized,
    {
        let resident_row = model::Entity::find()
            .filter(model::Column::GithubId.eq(id))
            .one(ctx.db())
            .await?;

        if let Some(ref m) = resident_row {
            if Utc::now() - m.last_updated < chrono::Duration::hours(6) {
                return Ok(Some(Resident {
                    id: m.github_id.try_into().expect("failed to convert number"),
                    name: m.name.clone(),
                    login: m.login.clone(),
                    avatar_url: m.avatar_url.clone(),
                }));
            }
        }

        let resident: Resident = match ctx
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

                return Err(ResolverError::GitHub);
            }
        };

        model::ActiveModel {
            id: resident_row
                .as_ref()
                .map(|m| ActiveValue::Unchanged(m.id))
                .unwrap_or(ActiveValue::NotSet),

            github_id: ActiveValue::Unchanged(
                resident.id.try_into().expect("failed to convert number"),
            ),

            login: ActiveValue::Set(resident.login.clone()),
            name: ActiveValue::Set(resident.name.clone()),
            avatar_url: ActiveValue::Set(resident.avatar_url.clone()),

            last_updated: ActiveValue::Set(Utc::now()),
        }
        .save(ctx.db())
        .await?;

        Ok(Some(resident))
    }

    async fn from_token(ctx: &BackendContext, token: &str) -> ResolverResult<(Resident, Octocrab)> {
        let github = Octocrab::builder()
            .personal_token(token.to_owned())
            .build()
            .expect("failed to create github client");

        let resident: Resident = github
            .get::<_, _, ()>("https://api.github.com/user", None)
            .await
            .map_err(|_| ResolverError::GitHub)?;

        let resident_row = model::Entity::find()
            .filter(model::Column::GithubId.eq(resident.id))
            .one(ctx.db())
            .await?;

        model::ActiveModel {
            id: resident_row
                .as_ref()
                .map(|m| ActiveValue::Unchanged(m.id))
                .unwrap_or(ActiveValue::NotSet),

            github_id: ActiveValue::Set(resident.id.try_into().expect("failed to convert number")),

            login: ActiveValue::Set(resident.login.clone()),
            name: ActiveValue::Set(resident.name.clone()),
            avatar_url: ActiveValue::Set(resident.avatar_url.clone()),

            last_updated: ActiveValue::Set(Utc::now()),
        }
        .save(ctx.db())
        .await?;

        Ok((resident, github))
    }
}

#[async_trait(?Send)]
impl QueryResolver for ResidentQuery {
    type Context = ResolverContext;

    async fn resolve(ctx: &ResolverContext, input: &Self::Input) -> QueryResult<Self> {
        let resident = match input {
            ResidentQueryInput::Id(github_id) => match Resident::get(ctx, *github_id).await? {
                Some(m) => m,
                None => return Err(RoutineError::NotFound),
            },
        };

        Ok(Self { content: resident }.into())
    }
}

#[async_trait(?Send)]
impl QueryResolver for CurrentResidentQuery {
    type Context = ResolverContext;

    async fn resolve(ctx: &ResolverContext, _input: &Self::Input) -> QueryResult<Self> {
        Ok(Self {
            content: ctx.resident().await?.cloned(),
        }
        .into())
    }
}

#[async_trait(?Send)]
impl MutationResolver for ExchangeTokenMutation {
    type Context = ResolverContext;

    async fn resolve(ctx: &ResolverContext, input: &Self::Input) -> MutationResult<Self> {
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
            .map_err(|_| ResolverError::GitHub)?;

        Ok(Self {
            content: access_token,
        }
        .into())
    }
}
