# skippable_map

[![Crates.io](https://img.shields.io/crates/v/skippable_map.svg?style=for-the-badge)](https://crates.io/crates/skippable_map)
[![Documentation](https://img.shields.io/docsrs/skippable_map?style=for-the-badge)](https://docs.rs/skippable_map/)
[![Build status](https://img.shields.io/github/actions/workflow/status/tveness/skippable_map/rust.yml?label=Tests&style=for-the-badge)](https://github.com/tveness/skippable_map/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/tveness/skippable_map?style=for-the-badge)](https://choosealicense.com/licenses/mit/)

This crate provides a wrapper around `HashMap` with a custom implementation of
`Deserialize` which skips any field which does not conform to the structure of the `HashMap`,
rather than throwing an error.
                                                                                                
This liberal approach to deserializing data is helpful if attempting to extract a subset of
information being passed in. For example a JSON blob with a mixed structure which cannot be
controlled, but a specific set of entries is of interest.
                                                                                                
# Example
                                                                                                
```rust
use serde_json;
use skippable_map::SkippableMap;
use std::collections::HashMap;
                                                                                                
let json = r#"{ "string": "b", "number": 1, "other_number": 2, "negative_number": -44}"#;
// SkippableMap<String, u64> will skip the (String, String) entry, and the negative number
let just_numbers: SkippableMap<String, u64> = serde_json::from_str(json).unwrap();
let hm = HashMap::from([
    (String::from("number"), 1_u64),
    (String::from("other_number"), 2_u64),
]);
                                                                                                
assert_eq!(just_numbers.as_ref(), &hm);
assert_eq!(just_numbers.0, hm);
// Consumes just_numbers to produce inner HashMap
assert_eq!(just_numbers.inner(), hm);
```


