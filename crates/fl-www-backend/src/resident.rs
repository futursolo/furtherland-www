use async_trait::async_trait;
use chrono::Utc;
pub(crate) use messages::Resident;
use octocrab::Octocrab;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::context::{RequestContext, ServerContext};
use crate::db::residents as model;
use crate::error::{HttpError, HttpResult};
use crate::prelude::*;

#[async_trait]
pub(crate) trait ResidentExt {
    async fn get(ctx: &RequestContext, id: u64) -> HttpResult<Option<Resident>>;
    async fn from_token(ctx: &ServerContext, token: &str) -> HttpResult<(Resident, Octocrab)>;
}

#[async_trait]
impl ResidentExt for Resident {
    async fn get(ctx: &RequestContext, id: u64) -> HttpResult<Option<Resident>> {
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

                return Err(HttpError::GitHub);
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

    async fn from_token(ctx: &ServerContext, token: &str) -> HttpResult<(Resident, Octocrab)> {
        let github = Octocrab::builder()
            .personal_token(token.to_owned())
            .build()
            .expect("failed to create github client");

        let resident: Resident = github
            .get::<_, _, ()>("https://api.github.com/user", None)
            .await
            .map_err(|_| HttpError::GitHub)?;

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
