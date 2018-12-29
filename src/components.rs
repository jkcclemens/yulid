//! Implementations for components of a [`Ulid`] that require `std`.

use crate::Ulid;

use chrono::{DateTime, TimeZone, Utc};

impl Ulid {
  /// Returns the timestamp portion of this [`Ulid`].
  pub fn as_timestamp(&self) -> DateTime<Utc> {
    Utc.timestamp_millis(self.as_millis())
  }

  /// Returns the timestamp portion of this [`Ulid`], capturing out-of-bounds values as `None`.
  pub fn as_timestamp_opt(&self) -> Option<DateTime<Utc>> {
    Utc.timestamp_millis_opt(self.as_millis()).single()
  }
}
