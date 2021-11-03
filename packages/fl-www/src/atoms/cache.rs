use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
// use std::ops::Deref;
use std::rc::Rc;

use bounce::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use web_sys::{Element, HtmlScriptElement};

use crate::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CacheState(HashMap<u64, serde_json::Value>);

impl Default for CacheState {
    fn default() -> Self {
        if let Some(m) = document().head() {
            let m: &Element = m.as_ref();

            if let Some(m) = m
                .query_selector("#fl-www-state-cache")
                .ok()
                .and_then(|m| m)
                .and_then(|m| m.dyn_into::<HtmlScriptElement>().ok())
                .and_then(|m| m.text().ok())
            {
                if let Ok(m) = serde_json::from_str::<Self>(&m) {
                    return m;
                }
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
        let mut h = DefaultHasher::new();
        k.hash(&mut h);
        let key = h.finish();

        self.0
            .get(&key)
            .cloned()
            .and_then(|m| serde_json::from_value(m).unwrap())
    }

    pub fn convert_action<K, V>(k: &K, v: V) -> Option<(u64, serde_json::Value)>
    where
        K: Hash,
        V: Serialize,
    {
        let mut h = DefaultHasher::new();
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
