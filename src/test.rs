extern crate test;

use crate::{Bytes, Ulid};

use self::test::Bencher;

const TEST_BYTES: Bytes = [
  1, 103, 245, 214, 154, 12, // timestamp
  107, 200, 228, 194, 102, 58, 236, 82, 247, 87, // random
];
const TEST_BASE32: &str = "05kzbnmt1hnwhs62crxermqqaw";
const TEST_MILLIS: i64 = 1546017741324;

const TEST_FIELD_1: u32 = 23590358;
const TEST_FIELD_2: u16 = 39436;
const TEST_FIELD_3: u16 = 27592;
const TEST_FIELD_4: u32 = 3837945402;
const TEST_FIELD_5: u32 = 3964860247;

#[bench]
fn bench_from_str(b: &mut Bencher) {
  b.iter(|| Ulid::parse_str(TEST_BASE32))
}

#[test]
fn from_fields() {
  let expected = Ulid::from_bytes(TEST_BYTES);
  let result = Ulid::from_fields(
    TEST_FIELD_1,
    TEST_FIELD_2,
    TEST_FIELD_3,
    TEST_FIELD_4,
    TEST_FIELD_5,
  );

  assert_eq!(
    expected,
    result,
  );
}

#[test]
fn as_fields() {
  let ulid = Ulid::from_bytes(TEST_BYTES);
  let (f1, f2, f3, f4, f5) = ulid.as_fields();

  assert_eq!(f1, TEST_FIELD_1);
  assert_eq!(f2, TEST_FIELD_2);
  assert_eq!(f3, TEST_FIELD_3);
  assert_eq!(f4, TEST_FIELD_4);
  assert_eq!(f5, TEST_FIELD_5);
}

#[test]
fn from_str() {
  let expected = Ulid::from_bytes(TEST_BYTES);
  let result = Ulid::parse_str(TEST_BASE32);

  assert_eq!(
    Ok(expected),
    result,
  );
}

#[test]
fn to_lowercase() {
  let ulid = Ulid::from_bytes(TEST_BYTES);

  assert_eq!(
    ulid.to_lowercase().to_string(),
    TEST_BASE32,
  );
}

#[test]
fn to_uppercase() {
  let ulid = Ulid::from_bytes(TEST_BYTES);

  assert_eq!(
    ulid.to_uppercase().to_string(),
    TEST_BASE32.to_uppercase(),
  );
}

#[test]
fn timestamp_millis() {
  let ulid = Ulid::from_bytes(TEST_BYTES);

  assert_eq!(
    ulid.as_millis(),
    TEST_MILLIS,
  );
}

#[cfg(feature = "uuid")]
mod uuid {
  use crate::Ulid;

  use uuid::Uuid;

  const TEST_UUID: [u8; 16] = [167, 60, 243, 221, 130, 30, 78, 250, 175, 236, 174, 157, 240, 232, 161, 205];

  #[test]
  fn ulid_from_uuid() {
    let uuid = Uuid::from_bytes(TEST_UUID);
    let ulid = Ulid::from(uuid);

    assert_eq!(
      uuid.as_bytes(),
      ulid.as_bytes(),
    );
  }

  #[test]
  fn uuid_from_ulid() {
    let ulid = Ulid::from_bytes(super::TEST_BYTES);
    let uuid = Uuid::from(ulid);

    assert_eq!(
      uuid.as_bytes(),
      ulid.as_bytes(),
    );
  }
}

#[cfg(feature = "serde")]
mod serde {
  extern crate serde_derive;

  use crate::Ulid;

  use self::serde_derive::{Deserialize, Serialize};

  const TEST_JSON: &str = r#"{"id":"05kzbnmt1hnwhs62crxermqqaw"}"#;
  const TEST_CBOR: &[u8] = &[161, 98, 105, 100, 80, 1, 103, 245, 214, 154, 12, 107, 200, 228, 194, 102, 58, 236, 82, 247, 87];

  #[derive(Debug, Deserialize, Serialize, PartialEq)]
  struct Test {
    id: Ulid,
  }

  #[test]
  fn to_json() {
    let ulid = Ulid::from_bytes(super::TEST_BYTES);

    let json = serde_json::to_string(&Test { id: ulid }).expect("could not serialise");

    assert_eq!(
      format!(r#"{{"id":"{}"}}"#, super::TEST_BASE32),
      json,
    );
  }

  #[test]
  fn from_json() {
    let ulid = Ulid::from_bytes(super::TEST_BYTES);
    let expected = Test {
      id: ulid,
    };

    let json: Test = serde_json::from_str(TEST_JSON).expect("could not deserialise");

    assert_eq!(
      json,
      expected,
    );
  }

  #[test]
  fn from_cbor() {
    let ulid = Ulid::from_bytes(super::TEST_BYTES);
    let expected = Test {
      id: ulid,
    };

    let cbor: Test = serde_cbor::from_slice(TEST_CBOR).expect("could not deserialise");

    assert_eq!(
      cbor,
      expected,
    );
  }

  #[test]
  fn to_cbor() {
    let ulid = Ulid::from_bytes(super::TEST_BYTES);
    let test = Test {
      id: ulid,
    };

    let result = serde_cbor::to_vec(&test).expect("could not serialise");

    assert_eq!(
      result,
      TEST_CBOR,
    );
  }
}

#[cfg(feature = "std")]
mod std_support {
  use crate::Ulid;

  use super::test::Bencher;

  use chrono::{TimeZone, Utc};

  #[bench]
  fn create_new(b: &mut Bencher) {
    b.iter(|| Ulid::new())
  }

  #[bench]
  fn create_static_timestamp(b: &mut Bencher) {
    let now = Utc::now();
    b.iter(|| Ulid::from_timestamp(now))
  }

  #[test]
  fn timestamp() {
    let ulid = Ulid::from_bytes(super::TEST_BYTES);
    let ts = Utc.timestamp_millis(super::TEST_MILLIS);

    assert_eq!(
      ulid.as_timestamp(),
      ts,
    );
  }
}
