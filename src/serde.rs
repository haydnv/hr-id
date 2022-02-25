use serde::de::{Deserialize, Deserializer, Error};
use serde::ser::{Serialize, Serializer};

use super::{validate_id, Id};

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let id = String::deserialize(deserializer)?;
        validate_id(&id).map_err(Error::custom)?;
        Ok(Self { id })
    }
}

impl Serialize for Id {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.id.serialize(serializer)
    }
}
