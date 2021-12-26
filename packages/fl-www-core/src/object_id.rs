use std::fmt;
use std::iter::repeat_with;
use std::str::FromStr;

use serde::de::{Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// A workaround until bson::oid::ObjectId works under wasm32.
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct ObjectId(
    #[serde(serialize_with = "ObjectId::serialise")]
    #[serde(deserialize_with = "ObjectId::deserialise")]
    Vec<u8>,
);

impl Default for ObjectId {
    fn default() -> ObjectId {
        let rng = fastrand::Rng::new();
        Self(repeat_with(|| rng.u8(..)).take(12).collect())
    }
}

impl ObjectId {
    pub fn new() -> Self {
        Self::default()
    }

    fn serialise<S>(inner: &[u8], ser: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_str(&hex::encode(inner))
    }

    fn deserialise<'de, D>(de: D) -> std::result::Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObjectIdVisitor;

        impl<'de> Visitor<'de> for ObjectIdVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an ObjectId")
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value
                    .parse()
                    .map_err(|e| E::custom(e))
                    .map(|m: ObjectId| m.0)
            }
        }

        de.deserialize_str(ObjectIdVisitor)
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
