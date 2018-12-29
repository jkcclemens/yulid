//! [`serde`] implementations for [`Ulid`].

use crate::Ulid;

use serde::{
  de::{self, Deserialize, Deserializer},
  ser::{Serialize, Serializer},
};

use core::fmt;

impl Serialize for Ulid {
  fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
  {
    if ser.is_human_readable() {
      return self.to_lowercase_ref().to_string().serialize(ser)
    }

    ser.serialize_bytes(self.as_bytes())
  }
}

impl<'de> Deserialize<'de> for Ulid {
  fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
  {
    if de.is_human_readable() {
      struct UlidStringVisitor;

      impl<'v> de::Visitor<'v> for UlidStringVisitor {
        type Value = Ulid;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "a ULID string")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Ulid, E> {
          value.parse().map_err(E::custom)
        }

        fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Ulid, E> {
          Ulid::from_slice(value).map_err(E::custom)
        }
      }

      return de.deserialize_str(UlidStringVisitor);
    }

    struct UlidByteVisitor;

    impl<'v> de::Visitor<'v> for UlidByteVisitor {
      type Value = Ulid;

      fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bytes")
      }

      fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Ulid, E> {
        Ulid::from_slice(value).map_err(E::custom)
      }
    }

    de.deserialize_bytes(UlidByteVisitor)
  }
}
