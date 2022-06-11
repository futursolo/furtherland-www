// use std::iter::repeat_with;

// use crate::prelude::*;

// pub fn sync() {
//     if let Some(stor) = window().local_storage().ok().and_then(|m| m) {
//         stor.set_item(
//             "fl_sync",
//             &repeat_with(fastrand::alphanumeric)
//                 .take(12)
//                 .collect::<String>(),
//         )
//         .expect("Failed to set item.");
//     }
// }
