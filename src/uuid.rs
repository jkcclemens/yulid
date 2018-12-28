use crate::Ulid;

use uuid::Uuid;

impl From<Uuid> for Ulid {
  fn from(uuid: Uuid) -> Self {
    Ulid::from_bytes(*uuid.as_bytes())
  }
}

impl From<Ulid> for Uuid {
  fn from(ulid: Ulid) -> Self {
    Uuid::from_bytes(*ulid.as_bytes())
  }
}
