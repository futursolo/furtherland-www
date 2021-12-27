use std::fmt;
use std::str::FromStr;

use rand::Rng;
use serde::de::{Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// A workaround until bson::oid::ObjectId works under wasm32.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct ObjectId(Vec<u8>);

impl Default for ObjectId {
    fn default() -> ObjectId {
        Self(rand::thread_rng().gen::<[u8; 12]>().into())
    }
}

impl Serialize for ObjectId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObjectIdVisitor;

        impl<'de> Visitor<'de> for ObjectIdVisitor {
            type Value = ObjectId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an ObjectId")
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value.parse().map_err(|e| E::custom(e))
            }
        }

        deserializer.deserialize_str(ObjectIdVisitor)
    }
}

impl ObjectId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FromStr for ObjectId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let seq = hex::decode(s).map_err(|_e| Error::ParseStr {
            target_kind: "ObjectId".to_owned(),
            reason: "failed to decode hex string.".to_owned(),
        })?;

        if seq.len() != 12 {
            return Err(Error::ParseStr {
                target_kind: "ObjectId".to_owned(),
                reason: "length mismatch.".to_owned(),
            });
        }

        Ok(Self(seq))
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}
