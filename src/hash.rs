use async_hash::{Digest, Hash, Output};

use super::Id;

impl<D: Digest> Hash<D> for Id {
    fn hash(self) -> Output<D> {
        Hash::<D>::hash(self.as_str())
    }
}

impl<'a, D: Digest> Hash<D> for &'a Id {
    fn hash(self) -> Output<D> {
        Hash::<D>::hash(self.as_str())
    }
}
