use anyhow::Context;
use futures::stream::{self, StreamExt, TryStreamExt};
use tokio::net::{self, TcpListener};
use tokio_stream::wrappers::TcpListenerStream;
use typed_builder::TypedBuilder;
use warp::Filter;

mod error;
use error::HttpError;

#[derive(Debug, PartialEq, TypedBuilder)]
pub struct WebServer {
    address: String,
}

impl WebServer {
    pub async fn run(self) -> anyhow::Result<()> {
        let s = net::lookup_host(&self.address)
            .await
            .map(stream::iter)
            .with_context(|| "failed to resolve host.")?
            .then(TcpListener::bind)
            .map_ok(TcpListenerStream::new)
            .try_flatten();

        let routes = warp::path::end().map(|| warp::reply::html("Hello World!"));

        let routes = routes
            // Cross-Origin Resource Sharing
            .with(warp::cors().allow_any_origin().build())
            // Error Handling
            .recover(HttpError::handle_rejection);

        warp::serve(routes).run_incoming(s).await;

        Ok(())
    }
}
