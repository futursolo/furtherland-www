use stellation_backend_tower::TowerRequest;

use crate::api::{
    create_resolver_registry, create_routine_registry, Bridge, Link, ResolverContext,
};

pub async fn create_backend_bridge(_req: TowerRequest<()>) -> Bridge {
    Bridge::new(
        Link::builder()
            .context(ResolverContext::builder().build())
            .resolvers(create_resolver_registry())
            .routines(create_routine_registry())
            .build(),
    )
}
