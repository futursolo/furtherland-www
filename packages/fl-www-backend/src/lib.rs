#![deny(clippy::all)]

use worker::{
    event, wasm_bindgen, wasm_bindgen_futures, worker_sys, Env, Request, Response, Router,
};

mod comment;
mod error;
mod logging;
mod prelude;
mod req_ctx;
mod user;

use error::Error;
use req_ctx::RequestContext;
use user::{User, UserExt};

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> worker::Result<Response> {
    logging::init();
    logging::log_request(&req);

    let token = req.headers().get("authorization")?;
    let token = match token {
        Some(m) => {
            if !m.to_lowercase().starts_with("bearer ") {
                return Ok(Error::Forbidden.into_response());
            }

            Some(m.chars().skip(7).collect::<String>())
        }

        None => None,
    };

    let req_ctx = if let Some(ref m) = token {
        let user = match User::from_token(m).await {
            Ok(m) => m,
            Err(e) => return Ok(e.into_response()),
        };

        RequestContext { user: Some(user) }
    } else {
        RequestContext { user: None }
    };

    let router = Router::with_data(req_ctx);
    let router = comment::register_endpoints(router);

    router
        .or_else_any_method("/*anything", |_, _| Ok(Error::NotFound.into_response()))
        .run(req, env)
        .await
}
