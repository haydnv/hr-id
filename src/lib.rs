//! A human-readable ID which is safe to use as a component in a URI path.
//! and supports constant [`Label`]s.
//!
//! Features:
//!  - `hash`: enable support for [`async-hash`](https://docs.rs/async-hash)
//!  - `serde`: enable support for [`serde`](https://docs.rs/serde)
//!  - `stream`: enable support for [`destream`](https://docs.rs/destream)
//!  - `uuid`: enable support for [`uuid`](https://docs.rs/uuid)
//!
//! Example:
//! ```
//! # use std::str::FromStr;
//! use hr_id::{label, Id, Label};
//!
//! const HELLO: Label = label("hello"); // unchecked!
//! let world: Id = "world".parse().expect("id");
//!
//! assert_eq!(format!("{}, {}!", HELLO, world), "hello, world!");
//! assert_eq!(Id::from(HELLO), "hello");
//! assert!(Id::from_str("this string has whitespace").is_err());
//! ```

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::mem::size_of;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use derive_more::*;
use get_size::GetSize;
use regex::Regex;
use safecast::TryCastFrom;

#[cfg(feature = "stream")]
mod destream;
#[cfg(feature = "hash")]
mod hash;
#[cfg(feature = "serde")]
mod serde;

/// A set of prohibited character patterns.
pub const RESERVED_CHARS: [&str; 21] = [
    "/", "..", "~", "$", "`", "&", "|", "=", "^", "{", "}", "<", ">", "'", "\"", "?", ":", "@",
    "#", "(", ")",
];

/// An error encountered while parsing an [`Id`].
#[derive(Debug, Display)]
#[display("{}", msg)]
pub struct ParseError {
    msg: Arc<str>,
}

impl std::error::Error for ParseError {}

impl From<String> for ParseError {
    fn from(msg: String) -> Self {
        Self { msg: msg.into() }
    }
}

impl From<&str> for ParseError {
    fn from(msg: &str) -> Self {
        Self { msg: msg.into() }
    }
}

/// A static label which implements `Into<Id>`.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Label {
    id: &'static str,
}

impl Deref for Label {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.id
    }
}

impl From<Label> for Id {
    fn from(l: Label) -> Id {
        Id { inner: l.id.into() }
    }
}

impl PartialEq<Id> for Label {
    fn eq(&self, other: &Id) -> bool {
        self.id == other.as_str()
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.id)
    }
}

/// Return a [`Label`] with the given static `str`.
pub const fn label(id: &'static str) -> Label {
    Label { id }
}

/// A human-readable ID
#[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Id {
    inner: Arc<str>,
}

impl Id {
    /// Borrows the String underlying this [`Id`].
    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_ref()
    }

    /// Destructure this [`Id`] into its inner `Arc<str>`.
    pub fn into_inner(self) -> Arc<str> {
        self.inner
    }

    /// Return true if this [`Id`] begins with the specified string.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.inner.starts_with(prefix)
    }
}

impl GetSize for Id {
    fn get_size(&self) -> usize {
        // err on the side of caution in case there is only one reference to this Id
        size_of::<Arc<str>>() + self.inner.as_bytes().len()
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for Id {
    fn from(id: uuid::Uuid) -> Self {
        Self {
            inner: id.to_string().into(),
        }
    }
}

impl Borrow<str> for Id {
    fn borrow(&self) -> &str {
        &self.inner
    }
}

impl PartialEq<String> for Id {
    fn eq(&self, other: &String) -> bool {
        self.inner.as_ref() == other.as_str()
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.inner.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for Id {
    fn eq(&self, other: &&'a str) -> bool {
        self.inner.as_ref() == *other
    }
}

impl PartialEq<Label> for Id {
    fn eq(&self, other: &Label) -> bool {
        self.inner.as_ref() == other.id
    }
}

impl PartialEq<Id> for &str {
    fn eq(&self, other: &Id) -> bool {
        *self == other.inner.as_ref()
    }
}

impl PartialOrd<String> for Id {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        self.inner.as_ref().partial_cmp(other.as_str())
    }
}

impl PartialOrd<str> for Id {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.inner.as_ref().partial_cmp(other)
    }
}

impl<'a> PartialOrd<&'a str> for Id {
    fn partial_cmp(&self, other: &&'a str) -> Option<Ordering> {
        self.inner.as_ref().partial_cmp(*other)
    }
}

impl From<usize> for Id {
    fn from(u: usize) -> Id {
        u.to_string().parse().expect("usize")
    }
}

impl From<u64> for Id {
    fn from(i: u64) -> Id {
        i.to_string().parse().expect("64-bit unsigned int")
    }
}

impl FromStr for Id {
    type Err = ParseError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        validate_id(id)?;

        Ok(Id { inner: id.into() })
    }
}

impl TryCastFrom<String> for Id {
    fn can_cast_from(id: &String) -> bool {
        validate_id(id).is_ok()
    }

    fn opt_cast_from(id: String) -> Option<Id> {
        id.parse().ok()
    }
}

impl TryCastFrom<Id> for usize {
    fn can_cast_from(id: &Id) -> bool {
        id.as_str().parse::<usize>().is_ok()
    }

    fn opt_cast_from(id: Id) -> Option<usize> {
        id.as_str().parse::<usize>().ok()
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.inner)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.inner)
    }
}

fn validate_id(id: &str) -> Result<(), ParseError> {
    if id.is_empty() {
        return Err("cannot construct an empty Id".into());
    }

    let mut invalid_chars = id.chars().filter(|c| (*c as u8) < 32u8);
    if let Some(invalid) = invalid_chars.next() {
        return Err(format!(
            "Id {} contains ASCII control characters {}",
            id, invalid as u8,
        )
        .into());
    }

    for pattern in &RESERVED_CHARS {
        if id.contains(pattern) {
            return Err(format!("Id {} contains disallowed pattern {}", id, pattern).into());
        }
    }

    if let Some(w) = Regex::new(r"\s").expect("whitespace regex").find(id) {
        return Err(format!("Id {} is not allowed to contain whitespace {:?}", id, w).into());
    }

    Ok(())
}
