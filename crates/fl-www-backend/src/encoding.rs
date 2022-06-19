use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde::Serialize;
use warp::filters::BoxedFilter;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::error::HttpError;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub(crate) enum Encoding {
    Json,
    Bincode,
}

impl Encoding {
    pub fn accept_filter() -> BoxedFilter<(Self,)> {
        warp::filters::header::optional::<String>("accept")
            .map(|m: Option<String>| {
                // Prefers json, but returns bincode when the application has indicated that they
                // accept bincode.

                let m = match m {
                    Some(m) => m,
                    None => return Self::Json,
                };

                for content_type in m.split(',').map(|m| m.trim().to_lowercase()) {
                    if content_type == "application/x-bincode" {
                        return Self::Bincode;
                    }
                }

                Self::Json
            })
            .boxed()
    }

    pub fn reply<R>(&self, b: &R) -> Response
    where
        R: Serialize,
    {
        match self {
            Self::Json => warp::reply::json(b).into_response(),
            Self::Bincode => warp::reply::with_header(
                bincode::serialize(b).expect("failed to serialize"),
                "content-type",
                "application/x-bincode",
            )
            .into_response(),
        }
    }

    pub fn request_body_filter<REQ>() -> BoxedFilter<(REQ,)>
    where
        REQ: 'static + Send + Sync + DeserializeOwned,
    {
        warp::filters::body::json::<REQ>()
            .or(warp::filters::header::optional::<String>("content-type")
                .map(|m: Option<String>| {
                    if m.map(|m| m.to_lowercase()).as_deref() != Some("application/x-bincode") {
                        return Err(warp::reject::custom(HttpError::UnsupportedMedia));
                    }

                    Ok(())
                })
                .and(warp::filters::body::bytes())
                .and_then(|_, m: Bytes| async move {
                    bincode::deserialize::<REQ>(&m)
                        .map_err(|_| warp::reject::custom(HttpError::BadRequest))
                }))
            .unify()
            .boxed()
    }
}
