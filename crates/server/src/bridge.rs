use stellation_backend::Request;
use stellation_backend_tower::TowerRequest;

use crate::api::{
    create_resolver_registry, create_routine_registry, Bridge, Link, ResolverContext,
};

pub async fn create_backend_bridge(req: TowerRequest<ResolverContext>) -> Bridge {
    let context = req.context().clone();

    Bridge::new(
        Link::builder()
            .context(context)
            .resolvers(create_resolver_registry())
            .routines(create_routine_registry())
            .build(),
    )
}
