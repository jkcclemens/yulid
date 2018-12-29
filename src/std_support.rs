use crate::{BytesError, ParseError, Ulid};

use std::str::FromStr;

impl FromStr for Ulid {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ulid::parse_str(s)
  }
}

impl std::error::Error for BytesError {}

impl std::error::Error for ParseError {}
