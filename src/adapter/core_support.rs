use crate::ParseError;

use core::fmt;

impl fmt::Display for crate::Ulid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.to_lowercase().fmt(f)
  }
}

impl fmt::Display for super::Lowercase {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.encode())
  }
}

impl<'a> fmt::Display for super::LowercaseRef<'a> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.encode())
  }
}

impl fmt::Display for super::Uppercase {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.encode())
  }
}

impl<'a> fmt::Display for super::UppercaseRef<'a> {
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
