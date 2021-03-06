//! Generate and parse ULIDs.
//!
//! Provides support for universally unique lexicographically sortable identifiers (ULIDs). A ULID
//! is a combination of a 48-bit timestamp and an 80-bit unique number, stored as 16 octets. ULIDs
//! are used to assign identifiers to entities without requiring a central allocating authority.
//!
//! They are particularly useful in distributed systems, though can be used in disparate areas, such
//! as databases and network protocols. Typically a UUID is displayed in a readable string form as a
//! sequence of 26 base32 digits.
//!
//! The uniqueness property is not strictly guaranteed, however for all practical purposes, it can
//! be assumed that an unintentional collision would be extremely unlikely.

#![cfg_attr(test, feature(test))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use byteorder::{BigEndian, ByteOrder};

#[cfg(test)]
mod test;

pub mod prelude;
pub mod parser;

#[cfg(feature = "std")]
pub mod generation;
#[cfg(feature = "std")]
pub mod components;
pub mod adapter;
mod core_support;
#[cfg(feature = "std")]
mod std_support;
#[cfg(feature = "uuid")]
mod uuid;
#[cfg(feature = "serde")]
mod serde;

pub use self::parser::ParseError;

/// A 128-bit (16 byte) buffer containing the ID.
pub type Bytes = [u8; 16];

/// A universally unique lexicographically sortable identifier (ULID).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ulid(Bytes);

impl Ulid {
  /// Creates a [`Ulid`] using the supplied bytes.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use yulid::{Bytes, Ulid};
  ///
  /// let bytes: Bytes = [1, 103, 245, 214, 154, 12, 107, 200, 228, 194, 102, 58, 236, 82, 247, 87];
  ///
  /// let ulid = Ulid::from_bytes(bytes);
  /// let ulid = ulid.to_lowercase().to_string();
  ///
  /// let expected_ulid = "05kzbnmt1hnwhs62crxermqqaw";
  ///
  /// assert_eq!(expected_ulid, ulid);
  /// ```
  ///
  /// An incorrect number of bytes:
  ///
  /// ```compile_fail
  /// use yulid::{Bytes, Ulid};
  ///
  /// let bytes: Bytes = [1, 2, 3, 4]; // doesn't compile
  ///
  /// let ulid = Ulid::from_bytes(bytes);
  /// ```
  pub const fn from_bytes(bytes: Bytes) -> Self {
    Ulid(bytes)
  }

  /// Creates a [`Ulid`] using the supplied bytes.
  ///
  /// # Errors
  ///
  /// This function will return an error if `slice` has any other length than 16.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use yulid::Ulid;
  ///
  /// let bytes = [1, 103, 245, 214, 154, 12, 107, 200, 228, 194, 102, 58, 236, 82, 247, 87];
  ///
  /// let ulid = Ulid::from_slice(&bytes);
  /// let ulid = ulid.map(|ulid| ulid.to_lowercase().to_string());
  ///
  /// let expected_ulid = Ok(String::from("05kzbnmt1hnwhs62crxermqqaw"));
  ///
  /// assert_eq!(expected_ulid, ulid);
  /// ```
  ///
  /// An incorrect number of bytes:
  ///
  /// ```
  /// use yulid::Ulid;
  ///
  /// let bytes = [1, 2, 3, 4];
  ///
  /// let ulid = Ulid::from_slice(&bytes);
  ///
  /// let expected_ulid = Err(yulid::BytesError::new(16, 4));
  ///
  /// assert_eq!(expected_ulid, ulid);
  /// ```
  pub fn from_slice(slice: &[u8]) -> Result<Self, BytesError> {
    let len = slice.len();
    if len != 16 {
      return Err(BytesError::new(16, len));
    }

    let mut bytes = [0; 16];
    bytes.copy_from_slice(slice);
    Ok(Ulid::from_bytes(bytes))
  }

  /// Creates a [`Ulid`] from milliseconds and the provided bytes.
  pub fn from_millis_bytes(millis: i64, mut bytes: [u8; 10]) -> Self {
    let mut buf = [0; 16];
    BigEndian::write_i48(&mut buf, millis);

    buf[6..].swap_with_slice(&mut bytes);

    Ulid::from_bytes(buf)
  }

  /// Creates a [`Ulid`] from a [`u128`] value.
  #[inline]
  pub fn from_u128(int: u128) -> Self {
    Ulid::from(int)
  }

  /// Creates a [`Ulid`] from five field values.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use yulid::Ulid;
  ///
  /// let (f1, f2, f3, f4, f5) = (23590358, 39436, 27592, 3837945402, 3964860247);
  ///
  /// let ulid = Ulid::from_fields(f1, f2, f3, f4, f5);
  /// let ulid = ulid.to_lowercase().to_string();
  ///
  /// let expected_ulid = "05kzbnmt1hnwhs62crxermqqaw";
  ///
  /// assert_eq!(expected_ulid, ulid);
  /// ```
  pub fn from_fields(f1: u32, f2: u16, f3: u16, f4: u32, f5: u32) -> Self {
    Ulid::from_bytes([
      (f1 >> 24) as u8,
      (f1 >> 16) as u8,
      (f1 >> 8) as u8,
      f1 as u8,
      (f2 >> 8) as u8,
      f2 as u8,
      (f3 >> 8) as u8,
      f3 as u8,
      (f4 >> 24) as u8,
      (f4 >> 16) as u8,
      (f4 >> 8) as u8,
      f4 as u8,
      (f5 >> 24) as u8,
      (f5 >> 16) as u8,
      (f5 >> 8) as u8,
      f5 as u8,
    ])
  }

  /// Returns the five field values of the [`Ulid`].
  ///
  /// These values can be passed to the [`Ulid::from_fields()`] method to get the original [`Ulid`]
  /// back.
  ///
  /// - The first field value represents the high 32 bits of the timestamp.
  /// - The second field value represents the low 16 bits of the timestamp.
  /// - The third field value represents the first 16 bits of the random portion.
  /// - The fourth field value represents the next 32 bits of the random portion.
  /// - The fifth field value represents the last 32 bits of the random portion.
  ///
  /// # Examples
  ///
  /// ```
  /// use yulid::Ulid;
  ///
  /// let ulid = Ulid::parse_str("05kzbnmt1hnwhs62crxermqqaw").unwrap();
  /// assert_eq!(
  ///   ulid.as_fields(),
  ///   (
  ///     23590358,
  ///     39436,
  ///     27592,
  ///     3837945402,
  ///     3964860247,
  ///   ),
  /// );
  /// ```
  pub fn as_fields(&self) -> (u32, u16, u16, u32, u32) {
    let f1 = u32::from(self.as_bytes()[0]) << 24
      | u32::from(self.as_bytes()[1]) << 16
      | u32::from(self.as_bytes()[2]) << 8
      | u32::from(self.as_bytes()[3]);

    let f2 = u16::from(self.as_bytes()[4]) << 8
      | u16::from(self.as_bytes()[5]);

    let f3 = u16::from(self.as_bytes()[6]) << 8
      | u16::from(self.as_bytes()[7]);

    let f4 = u32::from(self.as_bytes()[8]) << 24
      | u32::from(self.as_bytes()[9]) << 16
      | u32::from(self.as_bytes()[10]) << 8
      | u32::from(self.as_bytes()[11]);

    let f5 = u32::from(self.as_bytes()[12]) << 24
      | u32::from(self.as_bytes()[13]) << 16
      | u32::from(self.as_bytes()[14]) << 8
      | u32::from(self.as_bytes()[15]);

    (f1, f2, f3, f4, f5)
  }

  /// Returns the [`u128`] value represented by this [`Ulid`].
  #[inline]
  pub fn as_u128(self) -> u128 {
    self.into()
  }

  /// Returns an array of 16 octets containing the [`Ulid`] data.
  pub const fn as_bytes(&self) -> &Bytes {
    &self.0
  }

  /// Returns the milliseconds of the timestamp portion of the [`Ulid`].
  pub fn as_millis(&self) -> i64 {
    BigEndian::read_i48(self.as_bytes())
  }

  /// Parses a [`Ulid`] from a string of case-insensitive base32 digits.
  ///
  /// Any of the formats generated by this module (uppercase, lowercase) are supported by this
  /// parsing function.
  pub fn parse_str(input: &str) -> Result<Self, ParseError> {
    if input.len() != 26 {
      return Err(ParseError::InvalidLength {
        found: input.len(),
      })
    };
    let bytes = crate::parser::decode(input)?;
    // we know the slice is valid length
    Ok(Ulid::from_slice(&bytes).unwrap())
  }
}

/// The error that can occur when creating a [`Ulid`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BytesError {
  expected: usize,
  found: usize,
}

impl BytesError {
  /// Create a new [`BytesError`].
  pub const fn new(expected: usize, found: usize) -> Self {
    BytesError { expected, found }
  }

  /// The expected number of bytes.
  pub const fn expected(&self) -> usize {
    self.expected
  }

  /// The number of bytes found.
  pub const fn found(&self) -> usize {
    self.found
  }
}

impl From<u128> for Ulid {
  fn from(u: u128) -> Self {
    let mut bytes = [0; 16];
    BigEndian::write_u128(&mut bytes, u);

    Ulid::from_bytes(bytes)
  }
}

impl From<Ulid> for u128 {
  fn from(u: Ulid) -> Self {
    BigEndian::read_u128(u.as_bytes())
  }
}
