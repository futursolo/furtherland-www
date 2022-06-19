use async_trait::async_trait;
use sea_orm::entity::EntityTrait;

use crate::error::HttpResult;
#[async_trait]
pub(crate) trait ReplyExt {
    type Entity: EntityTrait;

    async fn create_reply() -> HttpResult<Self>
    where
        Self: Sized;
}
