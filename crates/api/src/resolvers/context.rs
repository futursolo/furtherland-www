use std::ops::Deref;
use std::sync::Arc;

use fl_www_backend::BackendContext;
use fl_www_core::messages::Resident;
use typed_builder::TypedBuilder;

use super::error::ResolverResult;

#[derive(Debug, TypedBuilder, Clone)]
pub struct ResolverContext {
    pub inner: Arc<BackendContext>,
}

impl ResolverContext {
    pub(crate) async fn resident(&self) -> ResolverResult<Option<&Resident>> {
        Ok(None)
    }
}

impl Deref for ResolverContext {
    type Target = BackendContext;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// impl RequestContext {
//     pub fn resident_github(&self) -> Option<&Octocrab> {
//         self.resident_github.as_ref()
//     }

//     pub fn github(&self) -> &Octocrab {
//         self.resident_github()
//             .unwrap_or_else(|| self.srv_ctx.github())
//     }

//     pub fn resident(&self) -> Option<&Resident> {
//         self.resident.as_ref()
//     }

//     pub fn reply<R>(&self, b: &R) -> Response
//     where
//         R: Serialize,
//     {
//         self.reply_encoding.reply(b)
//     }

//     pub fn filter(
//         ctx: Arc<ServerContext>,
//     ) -> impl Filter<Extract = (RequestContext,), Error = Rejection> + Send + Sync + Clone +
//       'static
//     {
//         warp::header::optional::<String>("authorization").and_then(move |token: Option<String>| {
//             let ctx = ctx.clone();

//             async move {
//                 match token.map(|m| {
//                     m.trim()
//                         .to_lowercase()
//                         .starts_with("bearer ")
//                         .then(|| {
//                             m.trim()
//                                 .chars()
//                                 .skip(7)
//                                 .collect::<String>()
//                                 .trim()
//                                 .to_owned()
//                         })
//                         .ok_or_else(|| Rejection::from(HttpError::Forbidden))
//                 }) {
//                     Some(Ok(m)) => {
//                         let (resident, github) = Resident::from_token(&ctx, &m).await?;

//                         Ok(RequestContext {
//                             srv_ctx: ctx.clone(),
//                             resident: Some(resident),
//                             resident_github: Some(github),
//                         })
//                     }
//                     Some(Err(e)) => Err(e),

//                     None => Ok(RequestContext {
//                         srv_ctx: ctx.clone(),
//                         resident: None,
//                         resident_github: None,
//                     }),
//                 }
//             }
//         })
//     }
// }
