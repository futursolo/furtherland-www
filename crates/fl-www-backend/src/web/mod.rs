use std::sync::Arc;

use anyhow::Context;
use futures::stream::{self, StreamExt, TryStreamExt};
use tokio::net::{self, TcpListener};
use tokio_stream::wrappers::TcpListenerStream;
use typed_builder::TypedBuilder;
use warp::Filter;

mod resident;

use crate::context::ServerContext;
use crate::error::HttpError;

#[derive(Debug, PartialEq, TypedBuilder)]
pub struct WebServer {
    address: String,
}

impl WebServer {
    pub async fn run(self) -> anyhow::Result<()> {
        let ctx = Arc::from(ServerContext::from_env());

        let s = net::lookup_host(&self.address)
            .await
            .map(stream::iter)
            .with_context(|| "failed to resolve host.")?
            .then(TcpListener::bind)
            .map_ok(TcpListenerStream::new)
            .try_flatten();

        let routes = warp::path::end()
            .map(|| warp::reply::html("Hello World!"))
            .or(resident::endpoints(ctx.clone()));

        let routes = // maximum request limit: 10MB
            warp::body::content_length_limit(10 * 1024 * 1024)
            .and(routes)
            // Cross-Origin Resource Sharing
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_header("content-type")
                    .allow_header("authorization")
                    .expose_header("content-type")
                    .build(),
            )
            // Error Handling
            .recover(HttpError::handle_rejection);

        warp::serve(routes).run_incoming(s).await;

        Ok(())
    }
}
