//! Provides a human-readable [`Id`] and static [`Label`]
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

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

// use async_hash::Hash; // TODO: add 'async-hash' feature flag
use regex::Regex;
use safecast::TryCastFrom;
// use sha2::digest::generic_array::{ArrayLength, GenericArray};
// use sha2::digest::{Digest, Output};

#[derive(Clone, Debug)]
pub struct Error(String);

impl std::error::Error for Error {}

pub type Result = std::result::Result<Id, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub const RESERVED_CHARS: [&str; 21] = [
    "/", "..", "~", "$", "`", "&", "|", "=", "^", "{", "}", "<", ">", "'", "\"", "?", ":", "@",
    "#", "(", ")",
];

/// A static label which implements `Into<Id>`.
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
        Id {
            id: l.id.to_string(),
        }
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

// TODO: enable this behind a feature flag
// impl From<uuid::Uuid> for Id {
//     fn from(id: uuid::Uuid) -> Self {
//         Id { id: id.to_string() }
//     }
// }

/// A human-readable `Id`
///
/// Must be valid UTF8 and must not contain whitespace or any [`RESERVED_CHARS`]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Id {
    id: String,
}

impl Id {
    // Construct an `Id` from a hexadecimal string representation of a SHA-2 hash.
    // pub fn from_hash<T, U>(hash: GenericArray<T, U>) -> Self
    //     where
    //         U: ArrayLength<T>,
    //         GenericArray<T, U>: AsRef<[u8]>,
    // {
    //     hex::encode(hash).parse().expect("hash")
    // }

    /// Borrows the String underlying this `Id`.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }

    /// Return true if this `Id` begins with the specified string.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.id.starts_with(prefix)
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.id == other
    }
}

impl<'a> PartialEq<&'a str> for Id {
    fn eq(&self, other: &&'a str) -> bool {
        self.id == *other
    }
}

impl PartialEq<Label> for Id {
    fn eq(&self, other: &Label) -> bool {
        self.id == other.id
    }
}

impl PartialEq<Id> for &str {
    fn eq(&self, other: &Id) -> bool {
        self == &other.id
    }
}

// impl<D: Digest> Hash<D> for Id {
//     fn hash(self) -> Output<D> {
//         Hash::<D>::hash(self.as_str())
//     }
// }
//
// impl<'a, D: Digest> Hash<D> for &'a Id {
//     fn hash(self) -> Output<D> {
//         Hash::<D>::hash(self.as_str())
//     }
// }

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
    type Err = Error;

    fn from_str(id: &str) -> Result {
        validate_id(id)?;
        Ok(Id { id: id.to_string() })
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

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

fn validate_id(id: &str) -> std::result::Result<(), Error> {
    if id.is_empty() {
        return Err(Error("Id cannot be empty".into()));
    }

    let filtered: &str = &id.chars().filter(|c| *c as u8 > 32).collect::<String>();
    if filtered != id {
        return Err(Error(format!(
            "Id {} contains an ASCII control character",
            id
        )));
    }

    for pattern in &RESERVED_CHARS {
        if id.contains(pattern) {
            return Err(Error(format!(
                "Id {} contains disallowed pattern {}",
                id, pattern
            )));
        }
    }

    if let Some(w) = Regex::new(r"\s").expect("whitespace regex").find(id) {
        return Err(Error(format!(
            "Id {} is not allowed to contain whitespace {:?}",
            id, w
        )));
    }

    Ok(())
}
