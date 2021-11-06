use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use ahash::AHasher;
use bounce::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use web_sys::{Element, HtmlScriptElement};

use crate::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CacheState(HashMap<u64, serde_json::Value>);

macro_rules! log_dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        ::log::debug!("[{}:{}]", $crate::file!(), $crate::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                ::log::debug!("[{}:{}] {} = {:#?}",
                    ::std::file!(), ::std::line!(), ::std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

impl Default for CacheState {
    fn default() -> Self {
        if let Some(m) = log_dbg!(log_dbg!(log_dbg!(document()
            .query_selector("#fl-www-state-cache")
            .ok())
        .and_then(|m| m)
        .and_then(|m| m.dyn_into::<HtmlScriptElement>().ok()))
        .and_then(|m| m.text().ok()))
        {
            if let Ok(m) = log_dbg!(serde_json::from_str::<Self>(&m)) {
                return log_dbg!(m);
            }
        }

        Self(HashMap::new())
    }
}

impl CacheState {
    fn update(&self) {
        if let Some(m) = document().head() {
            let m: &Element = m.as_ref();

            if let Ok(nodes) = m.query_selector_all("#fl-www-state-cache") {
                for i in 0..nodes.length() {
                    if let Some(node) = nodes.get(i) {
                        if let Some(m) = node.parent_node() {
                            m.remove_child(&node).unwrap();
                        }
                    }
                }
            }

            let next_cache = document()
                .create_element("script")
                .unwrap_throw()
                .dyn_into::<HtmlScriptElement>()
                .unwrap_throw();
            next_cache.set_type("application/json");

            next_cache.set_id("fl-www-state-cache");

            next_cache
                .set_text(&serde_json::to_string(self).unwrap_throw())
                .unwrap_throw();

            m.append_child(next_cache.as_ref()).unwrap();
        }
    }

    pub fn get<K, T>(&self, k: &K) -> Option<T>
    where
        K: Hash,
        T: DeserializeOwned,
    {
        let mut h = AHasher::new_with_keys(1234, 5678);
        k.hash(&mut h);
        let key = h.finish();

        let result = self
            .0
            .get(&key)
            .cloned()
            .and_then(|m| serde_json::from_value(m).unwrap());

        log::debug!("Cached: {}", result.is_some());

        result
    }

    pub fn convert_action<K, V>(k: &K, v: V) -> Option<(u64, serde_json::Value)>
    where
        K: Hash,
        V: Serialize,
    {
        let mut h = AHasher::new_with_keys(1234, 5678);
        k.hash(&mut h);
        let key = h.finish();

        Some((key, serde_json::to_value(v).ok()?))
    }
}

impl Slice for CacheState {
    type Action = (u64, serde_json::Value);

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        if self.0.contains_key(&action.0) {
            return self;
        }

        let mut self_ = self.clone_slice();

        self_.0.insert(action.0, action.1);

        self_.update();

        self_.into()
    }
}
