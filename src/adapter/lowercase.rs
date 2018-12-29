//! Adapters for formatting [`Ulid`](crate::Ulid)s as lowercase strings.

use crate::Ulid;

/// An adapter for formatting a [`Ulid`] as an lowercase string.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Lowercase(Ulid);

/// An adapter for formatting a [`Ulid`] as an lowercase string.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LowercaseRef<'a>(&'a Ulid);

impl Ulid {
  /// Creates a [`Lowercase`] instance from a [`Ulid`].
  pub const fn to_lowercase(self) -> Lowercase {
    Lowercase::from_ulid(self)
  }

  /// Creates a [`LowercaseRef`] instance from a [`Ulid`].
  pub const fn to_lowercase_ref(&self) -> LowercaseRef {
    LowercaseRef::from_ulid(self)
  }
}

impl Lowercase {
  /// Creates a [`Lowercase`] instance from a [`Ulid`].
  pub const fn from_ulid(ulid: Ulid) -> Self {
    Lowercase(ulid)
  }

  pub(crate) fn encode(self) -> String {
    crate::parser::encode(
      crate::parser::Case::Lower,
      self.0.as_bytes(),
    )
  }
}

impl<'a> LowercaseRef<'a> {
  /// Creates a [`LowercaseRef`] instance from a [`Ulid`].
  pub const fn from_ulid(ulid: &'a Ulid) -> Self {
    LowercaseRef(ulid)
  }

  pub(crate) fn encode(self) -> String {
    crate::parser::encode(
      crate::parser::Case::Lower,
      self.0.as_bytes(),
    )
  }
}
