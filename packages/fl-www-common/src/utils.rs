use std::convert::TryInto;
// use std::ops;
// use std::sync::{Arc, Mutex};

// use once_cell::sync::Lazy;

use crate::prelude::*;

// use yew_router::{
//     agent::{RouteAgentDispatcher, RouteRequest},
//     route::Route,
// };

//use crate::prelude::*;

// #[derive(PartialEq, Debug, Clone, Eq, Hash)]
// pub(crate) struct Id(u64);

// impl Id {
//     pub fn new() -> Self {
//         static CTR: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(0)));

//         let ctr = CTR.clone();
//         let mut ctr = ctr.lock().unwrap();

//         let current_ctr = *ctr;
//         (*ctr) += 1;

//         Self(current_ctr)
//     }
// }

// impl ops::Deref for Id {
//     type Target = u64;

//     fn deref(&self) -> &u64 {
//         &self.0
//     }
// }

// pub(crate) fn push_route(route: I18nRoute) {
//     let mut router: RouteAgentDispatcher<()> = RouteAgentDispatcher::new();
//     let route = Route::from(route);
//     router.send(RouteRequest::ChangeRoute(route));
// }

pub fn get_scroll_y() -> Option<u32> {
    let pos = document().document_element()?.scroll_top();

    if pos > 0 {
        let pos = pos.try_into().ok()?;
        return Some(pos);
    }

    let pos = document().body()?.scroll_top();

    if pos >= 0 {
        let pos = pos.try_into().ok()?;
        return Some(pos);
    }

    None
}
