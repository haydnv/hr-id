use std::str::FromStr;

use async_trait::async_trait;
use destream::de::{self, Decoder, FromStream};
use destream::en::{Encoder, IntoStream, ToStream};

use super::Id;

#[async_trait]
impl FromStream for Id {
    type Context = ();

    async fn from_stream<D: Decoder>(cxt: (), decoder: &mut D) -> Result<Self, D::Error> {
        let s = String::from_stream(cxt, decoder).await?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

impl<'en> ToStream<'en> for Id {
    fn to_stream<E: Encoder<'en>>(&'en self, e: E) -> Result<E::Ok, E::Error> {
        e.encode_str(self.as_str())
    }
}

impl<'en> IntoStream<'en> for Id {
    fn into_stream<E: Encoder<'en>>(self, e: E) -> Result<E::Ok, E::Error> {
        e.encode_str(self.as_str())
    }
}
