#![deny(clippy::all)]
#![deny(missing_debug_implementations)]

use std::sync::Arc;

use api::ResolverContext;
use backend::BackendContext;
use stellation_backend_cli::Cli;
use stellation_backend_tower::{TowerEndpoint, TowerRequest};
use {fl_www_api as api, fl_www_backend as backend, fl_www_view as view};

mod app;
mod bridge;
use app::ServerApp;
use bridge::create_backend_bridge;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(stellation_embedded_frontend)]
#[derive(rust_embed::RustEmbed)]
#[folder = "$STELLATION_FRONTEND_BUILD_DIR"]
struct Frontend;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configures Logging
    stellation_backend_cli::trace::init_default("STELLATION_APP_SERVER_LOG");

    let ctx: Arc<_> = BackendContext::from_env().await?.into();

    // let append_context = {
    //     let ctx = ctx.clone();

    //     move |req: TowerRenderRequest<()>| {
    //         let ctx = ctx.clone();

    //         async move { req.with_context(ResolverContext::builder().inner(ctx).build()) }
    //     }
    // };

    let create_bridge = {
        move |req: TowerRequest<()>| {
            let ctx = ctx.clone();
            async move {
                let req = req.with_context(ResolverContext::builder().inner(ctx).build());
                create_backend_bridge(req).await
            }
        }
    };

    // Creates Endpoint
    let endpoint = TowerEndpoint::<ServerApp<_>>::new()
        // .with_append_context(append_context)
        .with_create_bridge(create_bridge);

    #[cfg(stellation_embedded_frontend)]
    let endpoint =
        endpoint.with_frontend(stellation_backend_tower::Frontend::new_embedded::<Frontend>());

    // Starts Server
    Cli::builder().endpoint(endpoint).build().run().await?;

    Ok(())
}
