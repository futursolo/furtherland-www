#![deny(clippy::all)]

mod context;
pub mod db;
mod encoding;
mod error;
mod prelude;
mod reply;
mod resident;
mod web;

pub use web::WebServer;

// mod reply;
// mod req_ctx;
// mod resident;

// use req_ctx::RequestContext;
// use resident::{Resident, ResidentExt};

// fn affix_cors(mut resp: Response) -> Response {
//     let headers = resp.headers_mut();

//     headers
//         .set("Access-Control-Allow-Origin", "*")
//         .unwrap_throw();
//     headers
//         .set(
//             "Access-Control-Allow-Methods",
//             "GET, POST, PATCH, DELETE, OPTIONS",
//         )
//         .unwrap_throw();
//     headers
//         .set(
//             "Access-Control-Allow-Headers",
//             "Content-Type, Authorization",
//         )
//         .unwrap_throw();

//     resp
// }

// pub(crate) async fn options_cors(
//     _req: Request,
//     _ctx: worker::RouteContext<RequestContext>,
// ) -> crate::error::Result<Response> {
//     Ok(Response::empty()?.with_status(204))
// }

// #[event(fetch)]
// pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> worker::Result<Response> {
//     logging::init();
//     logging::log_request(&req);

//     let token = req.headers().get("authorization")?;
//     let token = match token {
//         Some(m) => {
//             if !m.to_lowercase().starts_with("bearer ") {
//                 return Ok(Error::Forbidden.into_response());
//             }

//             Some(m.chars().skip(7).collect::<String>())
//         }

//         None => None,
//     };

//     let req_ctx = if let Some(ref m) = token {
//         let resident = match Resident::from_token(m).await {
//             Ok(m) => m,
//             Err(e) => return Ok(e.into_response()),
//         };

//         RequestContext {
//             resident: Some(resident),
//         }
//     } else {
//         RequestContext { resident: None }
//     };

//     let router = Router::with_data(req_ctx);
//     let router = reply::register_endpoints(router);
//     let router = resident::register_endpoints(router);

//     router
//         .or_else_any_method("/*anything", |_, _| Ok(Error::NotFound.into_response()))
//         .run(req, env)
//         .await
//         .map(affix_cors)
// }
