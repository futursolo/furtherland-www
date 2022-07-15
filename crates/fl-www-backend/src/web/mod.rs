use std::sync::Arc;

use anyhow::Context;
use futures::stream::{self, StreamExt, TryStreamExt};
use tokio::net::{self, TcpListener};
use tokio_stream::wrappers::TcpListenerStream;
use typed_builder::TypedBuilder;
use warp::{Filter, Rejection, Reply};

mod exts;
mod replies;
mod residents;

use exts::FilterExt;

use crate::context::ServerContext;
use crate::error::{HttpError, HttpResult};

#[derive(Debug, PartialEq, TypedBuilder)]
pub struct WebServer {
    address: String,
}

impl WebServer {
    pub async fn run(self) -> anyhow::Result<()> {
        let ctx = Arc::from(ServerContext::from_env().await?);

        let s = net::lookup_host(&self.address)
            .await
            .map(stream::iter)
            .with_context(|| "failed to resolve host.")?
            .then(TcpListener::bind)
            .map_ok(TcpListenerStream::new)
            .try_flatten();

        let routes = warp::path::end()
            .map(|| warp::reply::html("Hello World!"))
            .or(residents::endpoints(ctx.clone()))
            .or(replies::endpoints(ctx));

        let cors = warp::cors()
            .allow_any_origin()
            .allow_methods(["GET", "POST", "OPTIONS", "HEAD", "PATCH", "DELETE"])
            .allow_header("content-type")
            .allow_header("authorization")
            .expose_header("content-type")
            .build();

        let content_limit = warp::filters::method::get()
            .or(warp::filters::method::head())
            .or(warp::filters::method::options())
            .or(warp::body::content_length_limit(10 * 1024 * 1024))
            .map(|_| ())
            .untuple_one();

        let routes = // maximum request limit: 10MB
            content_limit.and(routes)
            // Cross-Origin Resource Sharing
            .with(cors)
            // Error Handling
            .map(|m| HttpResult::Ok(Reply::into_response(m)))
            .recover(|err| async move { HttpError::recover_to_error(err).await.map_err(Rejection::from) })
            .unify()
            .terminated()
            .with(warp::log("fl_www_backend::web"));

        warp::serve(routes).run_incoming(s).await;

        Ok(())
    }
}
