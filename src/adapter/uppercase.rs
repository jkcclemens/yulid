//! Adapters for formatting [`Ulid`](crate::Ulid)s as uppercase strings.

use crate::Ulid;

/// An adapter for formatting a [`Ulid`] as an uppercase string.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Uppercase(Ulid);

/// An adapter for formatting a [`Ulid`] as an uppercase string.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UppercaseRef<'a>(&'a Ulid);

impl Ulid {
  /// Creates a [`Uppercase`] instance from a [`Ulid`].
  pub const fn to_uppercase(self) -> Uppercase {
    Uppercase::from_ulid(self)
  }

  /// Creates a [`UppercaseRef`] instance from a [`Ulid`].
  pub const fn to_uppercase_ref(&self) -> UppercaseRef {
    UppercaseRef::from_ulid(self)
  }
}

impl Uppercase {
  pub const fn from_ulid(ulid: Ulid) -> Self {
    Uppercase(ulid)
  }

  pub(crate) fn encode(self) -> String {
    crate::parser::encode(
      crate::parser::Case::Upper,
      self.0.as_bytes(),
    )
  }
}

impl<'a> UppercaseRef<'a> {
  pub const fn from_ulid(ulid: &'a Ulid) -> Self {
    UppercaseRef(ulid)
  }

  pub(crate) fn encode(self) -> String {
    crate::parser::encode(
      crate::parser::Case::Upper,
      self.0.as_bytes(),
    )
  }
}
