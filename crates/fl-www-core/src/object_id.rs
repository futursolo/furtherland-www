use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use serde::de::{Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// A workaround until bson::oid::ObjectId works under wasm32.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct ObjectId([u8; 12]);

impl Default for ObjectId {
    fn default() -> ObjectId {
        #[cfg(not(target_family = "wasm"))]
        let this = {
            use rand::Rng;
            Self(rand::thread_rng().gen::<[u8; 12]>())
        };

        #[cfg(target_family = "wasm")]
        let this = {
            let mut buf = [0_u8; 12];
            getrandom::getrandom(&mut buf).expect("failed to fill buffer");

            Self(buf)
        };

        this
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

        Ok(Self(seq.try_into().map_err(|_e| Error::ParseStr {
            target_kind: "ObjectId".to_owned(),
            reason: "length mismatch.".to_owned(),
        })?))
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}
