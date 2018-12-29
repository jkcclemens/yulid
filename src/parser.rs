//! [`Ulid`](crate::Ulid) parsing constructs and utilities.
//!
//! This is mostly base32 handling, which is a slightly modified version of the code in the
//! [base32 crate](https://crates.io/crates/base32).

use core::cmp::min;

const CROCKFORD: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const CROCKFORD_LOWER: &[u8] = b"0123456789abcdefghjkmnpqrstvwxyz";
const CROCKFORD_INV: [i8; 43] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31];

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

pub(crate) enum Case {
  Upper,
  Lower,
}

pub(crate) fn encode(casing: Case, data: &[u8]) -> String {
  let mut ret = Vec::with_capacity((data.len() + 3) / 4 * 5);

  let alphabet = match casing {
    Case::Upper => CROCKFORD,
    Case::Lower => CROCKFORD_LOWER,
  };

  for chunk in data.chunks(5) {
    let buf = {
      let mut buf = [0u8; 5];
      for (i, &b) in chunk.iter().enumerate() {
        buf[i] = b;
      }
      buf
    };
    ret.push(alphabet[((buf[0] & 0xF8) >> 3) as usize]);
    ret.push(alphabet[(((buf[0] & 0x07) << 2) | ((buf[1] & 0xC0) >> 6)) as usize]);
    ret.push(alphabet[((buf[1] & 0x3E) >> 1) as usize]);
    ret.push(alphabet[(((buf[1] & 0x01) << 4) | ((buf[2] & 0xF0) >> 4)) as usize]);
    ret.push(alphabet[(((buf[2] & 0x0F) << 1) | (buf[3] >> 7)) as usize]);
    ret.push(alphabet[((buf[3] & 0x7C) >> 2) as usize]);
    ret.push(alphabet[(((buf[3] & 0x03) << 3) | ((buf[4] & 0xE0) >> 5)) as usize]);
    ret.push(alphabet[(buf[4] & 0x1F) as usize]);
  }

  if data.len() % 5 != 0 {
    let len = ret.len();
    let num_extra = 8 - (data.len() % 5 * 8 + 4) / 5;
    ret.truncate(len - num_extra);
  }

  unsafe { String::from_utf8_unchecked(ret) }
}

pub(crate) fn decode(data: &str) -> Result<Vec<u8>, ParseError> {
  let data = data.as_bytes();
  let mut unpadded_data_length = data.len();
  for i in 1..=min(6, data.len()) {
    if data[data.len() - i] != b'=' {
      break;
    }
    unpadded_data_length -= 1;
  }
  let output_length = unpadded_data_length * 5 / 8;
  let mut ret = Vec::with_capacity((output_length + 4) / 5 * 5);
  for chunk in data.chunks(8) {
    let buf = {
      let mut buf = [0u8; 8];
      for (i, &c) in chunk.iter().enumerate() {
        if !c.is_ascii() {
          return Err(ParseError::InvalidCharacter {
            found: c as char,
            index: i,
          });
        }
        match CROCKFORD_INV.get(c.to_ascii_uppercase().wrapping_sub(b'0') as usize) {
          Some(&-1) | None => return Err(ParseError::InvalidCharacter {
            found: c as char,
            index: i,
          }),
          Some(&value) => buf[i] = value as u8,
        };
      }
      buf
    };
    ret.push((buf[0] << 3) | (buf[1] >> 2));
    ret.push((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4));
    ret.push((buf[3] << 4) | (buf[4] >> 1));
    ret.push((buf[4] << 7) | (buf[5] << 2) | (buf[6] >> 3));
    ret.push((buf[6] << 5) | buf[7]);
  }
  ret.truncate(output_length);
  Ok(ret)
}

/// An error that can occur while parsing a [`Ulid`](crate::Ulid) string.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
  /// Invalid character in the [`Ulid`](crate::Ulid) string.
  InvalidCharacter {
    /// The invalid character found.
    found: char,
    /// The invalid character position.
    index: usize,
  },
  /// Invalid length of the [`Ulid`](crate::Ulid) string.
  InvalidLength {
    /// The invalid length found.
    found: usize,
  },
}

impl ParseError {
  pub(crate) fn _description(&self) -> &str {
    match *self {
      ParseError::InvalidCharacter { .. } => "invalid character",
      ParseError::InvalidLength { .. } => "invalid length",
    }
  }
}
