use std::str::FromStr;

use async_trait::async_trait;
use destream::de::{self, Decoder, FromStream};
use destream::en::{Encoder, IntoStream, ToStream};

use super::Id;

struct IdVisitor;

#[async_trait]
impl de::Visitor for IdVisitor {
    type Value = Id;

    fn expecting() -> &'static str {
        "a human-readable Id"
    }

    fn visit_string<E: de::Error>(self, s: String) -> Result<Self::Value, E> {
        Id::from_str(&s).map_err(de::Error::custom)
    }

    async fn visit_map<M: de::MapAccess>(self, mut access: M) -> Result<Self::Value, M::Error> {
        if let Some(key) = access.next_key::<String>(()).await? {
            let value: [u8; 0] = access.next_value(()).await?;
            if value.is_empty() {
                Id::from_str(&key).map_err(de::Error::custom)
            } else {
                Err(de::Error::custom("Expected Id but found OpRef"))
            }
        } else {
            Err(de::Error::custom("Unable to parse Id"))
        }
    }
}

#[async_trait]
impl FromStream for Id {
    type Context = ();

    async fn from_stream<D: Decoder>(_context: (), d: &mut D) -> Result<Self, D::Error> {
        d.decode_any(IdVisitor).await
    }
}

impl<'en> ToStream<'en> for Id {
    fn to_stream<E: Encoder<'en>>(&'en self, e: E) -> Result<E::Ok, E::Error> {
        e.encode_str(&self.id)
    }
}

impl<'en> IntoStream<'en> for Id {
    fn into_stream<E: Encoder<'en>>(self, e: E) -> Result<E::Ok, E::Error> {
        e.encode_str(&self.id)
    }
}
