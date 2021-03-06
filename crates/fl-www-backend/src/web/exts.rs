use warp::filters::BoxedFilter;
use warp::reply::Response;
use warp::{Filter, Rejection, Reply};

use crate::encoding::Encoding;
use crate::error::HttpResult;

pub trait FilterExt: Filter {
    /// Terminate an HttpError into a Response.
    fn terminated(self) -> BoxedFilter<(Response,)>;
}

impl<T, F> FilterExt for F
where
    T: 'static + Reply,
    F: Filter<Extract = (HttpResult<T>,), Error = Rejection> + Send + Sync + Clone + 'static,
{
    fn terminated(self) -> BoxedFilter<(Response,)> {
        self.and(Encoding::accept_filter())
            .then(|m: HttpResult<T>, reply_encoding: Encoding| async move {
                m.map(|m| m.into_response())
                    .unwrap_or_else(|e| e.to_reply(reply_encoding).into_response())
            })
            .boxed()
    }
}
