use serde::{Deserialize, Serialize};
use worker::{Request, Response, RouteContext, Router};

use crate::error::{Error, Result};
use crate::prelude::*;
use crate::user::{User, UserExt};
use crate::RequestContext;
use object_id::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CommentValue {
    id: ObjectId,
    slug: String,
    user_id: u64,
    content: String,
}

pub(crate) async fn post_comment(
    mut req: Request,
    ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    let user = match ctx.data().user {
        Some(ref m) => m.clone(),
        None => return Err(Error::Forbidden),
    };

    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let input = req
        .json::<messages::CommentInput>()
        .await
        .map_err(|_e| Error::BadRequest)?;

    let comment_store = ctx.kv("COMMENTS")?;

    let id = ObjectId::new();
    let key = format!("{}:{}", slug, id);

    let comment = CommentValue {
        id,
        slug: slug.to_owned(),
        user_id: user.id,
        content: input.content,
    };

    comment_store.put(&key, &comment)?.execute().await?;

    let resp = messages::Response::Success {
        content: messages::Comment {
            id: comment.id,
            slug: comment.slug,
            content: comment.content,
            user: Some(user),
        },
    };

    Ok(Response::from_json(&resp)?)
}

pub(crate) async fn get_comments(
    _req: Request,
    ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let comment_store = ctx.kv("COMMENTS")?;

    // TODO: pagination.
    let comment_list = comment_store
        .list()
        .prefix(format!("{}:", slug))
        .execute()
        .await?;

    let mut comments = Vec::new();
    for key in comment_list.keys.iter() {
        let comment = match comment_store.get(&key.name).await? {
            Some(m) => m,
            None => continue,
        };

        let comment_value = comment.as_json::<CommentValue>()?;

        let maybe_user = User::get(&ctx, comment_value.user_id).await?;

        let comment = messages::Comment {
            id: comment_value.id,
            slug: comment_value.slug,
            content: comment_value.content,
            user: maybe_user,
        };

        comments.push(comment);
    }

    let comments = messages::Comments {
        comments,
        cursor: comment_list.cursor,
    };

    let resp = messages::Response::Success { content: comments };

    Ok(Response::from_json(&resp)?)
}

pub(crate) async fn get_comment(
    _req: Request,
    ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    let comment_store = ctx.kv("COMMENTS")?;

    let slug = match ctx.param("slug") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let comment_id = match ctx.param("id") {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };
    let comment = match comment_store
        .get(&format!("{}:{}", slug, comment_id))
        .await?
    {
        Some(m) => m,
        None => return Err(Error::NotFound),
    };

    let comment_value = comment.as_json::<CommentValue>()?;

    let maybe_user = User::get(&ctx, comment_value.user_id).await?;

    let comment = messages::Comment {
        id: comment_value.id,
        slug: comment_value.slug,
        content: comment_value.content,
        user: maybe_user,
    };

    let resp = messages::Response::Success { content: comment };

    Ok(Response::from_json(&resp)?)
}

pub(crate) async fn patch_comment(
    _req: Request,
    _ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    todo!()
}

pub(crate) async fn delete_comment(
    _req: Request,
    _ctx: RouteContext<RequestContext>,
) -> Result<Response> {
    todo!()
}

pub(crate) fn register_endpoints(router: Router<'_, RequestContext>) -> Router<'_, RequestContext> {
    router
        .post_async("/comments/:slug/", |m, n| async move {
            Ok(post_comment(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .get_async("/comments/:slug/", |m, n| async move {
            Ok(get_comments(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .get_async("/comments/:slug/:id", |m, n| async move {
            Ok(get_comment(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .patch_async("/comments/:slug/:id", |m, n| async move {
            Ok(patch_comment(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
        .delete_async("/comments/:slug/:id", |m, n| async move {
            Ok(delete_comment(m, n)
                .await
                .unwrap_or_else(|e| e.into_response()))
        })
}
