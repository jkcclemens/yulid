use crate::{
  Ulid,
  BytesError, ParseError,
  adapter::{
    Lowercase, LowercaseRef,
    Uppercase, UppercaseRef,
  },
};

use core::fmt;

impl fmt::Display for Ulid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.to_lowercase().fmt(f)
  }
}

impl fmt::Display for Lowercase {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.encode())
  }
}

impl<'a> fmt::Display for LowercaseRef<'a> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.encode())
  }
}

impl fmt::Display for Uppercase {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.encode())
  }
}

impl<'a> fmt::Display for UppercaseRef<'a> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.encode())
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: ", self._description())?;

    match *self {
      ParseError::InvalidCharacter { found, index } => write!(
        f,
        "expected valid base32, found {} at index {}",
        found,
        index,
      ),
      ParseError::InvalidLength { found } => write!(
        f,
        "expected 26, found {}",
        found,
      ),
    }
  }
}

impl fmt::Display for BytesError {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "invalid bytes length: expected {}, found {}",
      self.expected(),
      self.found(),
    )
  }
}
