use async_hash::generic_array::{ArrayLength, GenericArray};
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

impl<T, U> From<GenericArray<T, U>> for Id
where
    U: ArrayLength<T>,
    GenericArray<T, U>: AsRef<[u8]>,
{
    fn from(hash: GenericArray<T, U>) -> Self {
        hex::encode(hash).parse().expect("hash")
    }
}
