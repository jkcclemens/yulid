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
