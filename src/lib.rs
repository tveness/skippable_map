//! This crate provides a wrapper around [`HashMap`] with a custom implementation of
//! [`Deserialize`] which skips any field which does not conform to the structure of the `HashMap`,
//! rather than throwing an error.
//!
//! This liberal approach to deserializing data is helpful if attempting to extract a subset of
//! information being passed in. For example a JSON blob with a mixed structure which cannot be
//! controlled, but a specific set of entries is of interest.
//!
//! # Example
//!
//! ```rust
//! use serde_json;
//! use skippable_map::SkippableMap;
//! use std::collections::HashMap;
//!
//! let json = r#"{ "string": "b", "number": 1, "other_number": 2, "negative_number": -44}"#;
//! // SkippableMap<String, u64> will skip the (String, String) entry, and the negative number
//! let just_numbers: SkippableMap<String, u64> = serde_json::from_str(json).unwrap();
//! let hm = HashMap::from([
//!     (String::from("number"), 1_u64),
//!     (String::from("other_number"), 2_u64),
//! ]);
//!
//! assert_eq!(just_numbers.as_ref(), &hm);
//! assert_eq!(just_numbers.0, hm);
//! // Consumes just_numbers to produce inner HashMap
//! assert_eq!(just_numbers.inner(), hm);
//! ```

use serde::{de::Visitor, Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData};

/// The central struct of the library: this is a wrapper around [`HashMap`] with a custom
/// implementation of [`Deserialize`].
/// The implementation goes through the data to be deserialized, and skips any field which does not
/// conform to the `HashMap<K,V>` format.
///
/// This means that we can pass a data structure with additional components not in this format
/// which will be skipped.
///
/// # Examples
///
/// ```rust
/// use serde_json;
/// use skippable_map::SkippableMap;
/// use std::collections::HashMap;

/// let json = r#"{ "string": "b", "number": 1, "other_number": 2, "negative_number": -44}"#;
/// // SkippableMap<String, u64> will skip the (String, String) entry, and the negative number
/// let just_numbers: SkippableMap<String, u64> = serde_json::from_str(json).unwrap();
/// let hm = HashMap::from([
///     (String::from("number"), 1_u64),
///     (String::from("other_number"), 2_u64),
/// ]);
///
/// assert_eq!(just_numbers.0, hm);
/// ```
#[derive(Debug, Clone, Default, Serialize)]
#[serde(transparent)]
pub struct SkippableMap<K, V>(pub HashMap<K, V>);

impl<K, V> SkippableMap<K, V> {
    pub fn inner(self) -> HashMap<K, V> {
        self.0
    }
}

struct SkippableMapVisitor<K, V> {
    marker: PhantomData<fn() -> SkippableMap<K, V>>,
}

impl<K, V> SkippableMapVisitor<K, V> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for SkippableMapVisitor<K, V>
where
    K: Deserialize<'de> + std::hash::Hash + std::cmp::Eq,
    V: Deserialize<'de>,
{
    type Value = SkippableMap<K, V>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a data structure which contains some mappings from {} to {}",
            std::any::type_name::<K>(),
            std::any::type_name::<V>(),
        )
    }

    fn visit_map<A>(self, mut access: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        // Skips any entries which don't decode to map from K to V
        loop {
            let r = access.next_entry();
            match r {
                // Success in decoding (insert)
                Ok(Some((key, value))) => {
                    map.insert(key, value);
                }
                // Error in decoding (skip)
                Err(_) => {}
                // End of data structure (end)
                Ok(None) => {
                    return Ok(SkippableMap(map));
                }
            };
        }
    }
}

impl<K, V> From<SkippableMap<K, V>> for HashMap<K, V> {
    fn from(value: SkippableMap<K, V>) -> Self {
        value.0
    }
}

impl<K, V> AsRef<HashMap<K, V>> for SkippableMap<K, V> {
    fn as_ref(&self) -> &HashMap<K, V> {
        &self.0
    }
}

impl<'de, K, V> Deserialize<'de> for SkippableMap<K, V>
where
    K: Deserialize<'de> + std::hash::Hash + std::cmp::Eq,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(SkippableMapVisitor::new())
    }
}
