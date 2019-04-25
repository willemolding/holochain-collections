# Holochain-Collections
A helper crate for implementing collections of entries in Holochain

! This is currently in very early development state. Please do not use other than for testing purposes !

## Implemented collections

### Bucket Set

- Immutable
- Static
- unordered/partially ordered

This collection exists to solve the DHT hotspot problem. It works by taking a large collection of links that would be added to a single base and spreading them over a number of buckets according to a deterministic function of the entry. This reduces the load on the holders of the base and improves scaleability at the cost of increased retriveal complexity.

To use this collection you must implement the `BucketSetStorable` trait on your entry struct. This requires the `derive_bucket_id` function which takes the entry and must return a string. This determines which bucket the entry will be added to. For example if the entry had a string field you could split buckets by the first character e.g.
```rust
use holochain_collections::bucket_set::{
    self,
    BucketSetStorable,
}

// create an example entry struct and implement BucktSetStorable
#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct MyEntry {
    content: String,
}

impl BucketSetStorable for MyEntry {
    fn derive_bucket_id(&self) -> String {
        self.content.chars().next().unwrap_or('\0').to_string()
    }
}
```

The `bucket_set::store()` and `bucket_set::retrieve()` are then available to use with any entry assicociated with this struct.

```rust

pub fn handle_create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
    let address = bucket_set::store("my_entry".into(), entry)?;
    Ok(address)
}

pub fn handle_get_entries_by_bucket(bucket_id: String) -> ZomeApiResult<Vec<Address>> {
    bucket_set::retrieve("my_entry".into(), bucket_id)
}

```

The only other consideration you need to make as a developer is to add the special bucket entry to the `define_zome!` macro.

```rust
define_zome! {
    entries: [
       definition(),
       bucket_set::bucket_entry_def_for("my_entry".into()) // add the special bucket entry
    ]
    // snip
}
```

If you want to ensure uniform distribution across the buckets it best to use the prefix of the hash as the bucket identifier. The bucket_set module provides a helper function for this case.

```rust
impl BucketSetStorable for MyEntry {
    fn derive_bucket_id(&self) -> String {
        let entry = Entry::App("my_entry".into(), self.into());
        bucket_set::bucket_id_from_hash_prefix(entry, 3) // use three bits of hash prefix (2^3 = 8 buckets)
    }
}
```

If you want to be able to call `retrieve_all` you must provide a way to exhaustively iterate over all possible bucket ids. This is done by implementing `BucketIterable` on your entry struct. Following the example above of using the first character of a string as the bucket_id. Lets only allow the english alphabet for this example

```rust
impl BucketIterable for MyEntry {
    fn buckets() -> Box<Iterator<Item = String>> {
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ"
        .chars().map(|c| {
            c.to_string()
        });
        Box::new(alphabet)
    }
}
``` 