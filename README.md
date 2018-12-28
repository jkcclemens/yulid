# yulid

A Rust implementation of the [ULID spec](https://github.com/ulid/spec) that aims to be as similar to
[`uuid`](https://crates.io/crates/uuid) as possible.

## Supports

- Generation with `rand` API (by default)
- Serialisation and deserialisation with `serde` (with feature)
- Converting to and from UUIDs provided by the `uuid` crate (with feature)

## Examples

```rust
use yulid::Ulid;

fn main() {
  // create a new ULID
  let ulid = Ulid::new();

  // print the lowercase form
  println!("{}", ulid.to_lowercase());

  // get the DateTime<Utc> this ULID contains
  let timestamp = ulid.timestamp();
}
```

```rust
use rand::{Rng, thread_rng};
use yulid::Ulid;

fn main() {
  // generate a ULID using rand
  let ulid: Ulid = thread_rng().gen();
}
```
