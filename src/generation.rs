//! Implementations for [`Ulid`] generation.
//!
//! Only available with the `std` feature.

use crate::Ulid;

use byteorder::{BigEndian, ByteOrder};
use chrono::{DateTime, Utc};
use rand::{
  distributions::{Distribution, Standard},
  Rng,
  thread_rng,
};

impl Ulid {
  /// Creates a random [`Ulid`] with the current timestamp.
  ///
  /// This uses the [`rand`] crate's default task RNG as the source of random numbers. If you'd like
  /// to use a custom generator, don't use this method: use either the [`Ulid::from_rng()`] method or the
  /// `gen` method on `rand`'s [`Rng`].
  #[inline]
  pub fn new() -> Self {
    Ulid::from_rng(&mut thread_rng())
  }

  /// Creates a random [`Ulid`] with the current timestamp, using a custom source of randomness.
  pub fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
    // get the timestamp portion of the ulid
    let millis = Utc::now().timestamp_millis();

    // create the buffer holding the raw bytes
    let mut buf = [0; 16];

    // write the timestamp section into the buffer
    BigEndian::write_i48(&mut buf, millis);

    // fill the rest of the buffer with random bytes
    rng.fill(&mut buf[6..]);

    // construct the resulting ulid
    Ulid(buf)
  }

  /// Creates a [`Ulid`] from a timestamp.
  ///
  /// This function will use the provided timestamp for the timestamp portion of the [`Ulid`], and
  /// the [`rand`] crate's default task RNG will be used for the random portion.
  ///
  /// To use a custom source of randomness with a timestamp, see
  /// [`Ulid::from_timestamp_with_rng()`].
  #[inline]
  pub fn from_timestamp(timestamp: DateTime<Utc>) -> Self {
    Ulid::from_timestamp_with_rng(timestamp, &mut thread_rng())
  }

  /// Creates a [`Ulid`] from a timestamp and a custom RNG.
  ///
  /// This function will use the provided timestamp for the timestamp portion of the [`Ulid`], and
  /// the provided [`Rng`] will be used for the random portion.
  #[inline]
  pub fn from_timestamp_with_rng<R: Rng + ?Sized>(timestamp: DateTime<Utc>, rng: &mut R) -> Self {
    Ulid::from_millis_with_rng(timestamp.timestamp_millis(), rng)
  }

  /// Creates a [`Ulid`] from a timestamp and the provided bytes.
  #[inline]
  pub fn from_timestamp_bytes(timestamp: DateTime<Utc>, bytes: [u8; 10]) -> Self {
    Ulid::from_millis_bytes(timestamp.timestamp_millis(), bytes)
  }

  /// Creates a [`Ulid`] from milliseconds.
  ///
  /// This function will use the provided milliseconds for the timestamp portion of the [`Ulid`],
  /// and the [`rand`] crate's default task RNG will be used for the random portion.
  ///
  /// To use a custom source of randomness with milliseconds, see [`Ulid::from_millis_with_rng()`].
  #[inline]
  pub fn from_millis(millis: i64) -> Self {
    Ulid::from_millis_with_rng(millis, &mut thread_rng())
  }

  /// Creates a [`Ulid`] from milliseconds and a custom RNG.
  ///
  /// This function will use the provided milliseconds for the timestamp portion of the [`Ulid`],
  /// and the provided [`Rng`] will be used for the random portion.
  #[inline]
  pub fn from_millis_with_rng<R: Rng + ?Sized>(millis: i64, rng: &mut R) -> Self {
    let mut buf = [0; 10];
    rng.fill(&mut buf);

    Ulid::from_millis_bytes(millis, buf)
  }
}

impl Distribution<Ulid> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ulid {
    Ulid::from_rng(rng)
  }
}
