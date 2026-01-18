//! Deep merge trait for configuration objects.
//!
//! The `Merge` trait provides Nickel-style deep merging (like the `&` operator)
//! for Rust types. This enables combining base configurations with overlays.
//!
//! # Merge Semantics
//!
//! - **Primitives and `Option<T>`**: The "other" value wins if it's `Some`, otherwise keep base
//! - **Structs**: Recursively merge each field
//! - **Maps/`HashMap`**: Merge entries, with "other" values winning on key conflicts
//! - **Arrays/`Vec`**: Replace entirely (no element-wise merge by default)
//!
//! # Example
//!
//! ```rust
//! use amalgam_runtime::Merge;
//!
//! #[derive(Debug, Clone, Default)]
//! struct Config {
//!     name: Option<String>,
//!     replicas: Option<i32>,
//! }
//!
//! impl Merge for Config {
//!     fn merge(self, other: Self) -> Self {
//!         Self {
//!             name: other.name.or(self.name),
//!             replicas: other.replicas.or(self.replicas),
//!         }
//!     }
//! }
//!
//! let base = Config { name: Some("app".into()), replicas: Some(1) };
//! let overlay = Config { name: None, replicas: Some(3) };
//! let merged = base.merge(overlay);
//!
//! assert_eq!(merged.name, Some("app".into()));  // Base preserved
//! assert_eq!(merged.replicas, Some(3));          // Overlay wins
//! ```

use std::collections::{BTreeMap, HashMap};

/// Trait for deep merging configuration objects.
///
/// This mirrors Nickel's `&` operator semantics, enabling compositional
/// configuration where overlays can selectively override base values.
pub trait Merge: Sized {
    /// Merge `other` into `self`, with `other` taking precedence.
    ///
    /// This consumes both values and returns the merged result.
    fn merge(self, other: Self) -> Self;

    /// Merge with a closure that modifies a mutable reference.
    ///
    /// This is a convenience method for inline modifications:
    ///
    /// ```rust,ignore
    /// let config = base
    ///     .merge_with(|c| c.replicas = Some(3))
    ///     .merge_with(|c| c.name = Some("updated".into()));
    /// ```
    fn merge_with<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }

    /// Merge multiple overlays in sequence.
    ///
    /// Later overlays take precedence over earlier ones.
    fn merge_all<I>(self, overlays: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        overlays.into_iter().fold(self, |acc, overlay| acc.merge(overlay))
    }
}

// --- Implementations for standard types ---

impl<T> Merge for Option<T>
where
    T: Merge,
{
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.merge(b)),
            (a, b) => b.or(a),
        }
    }
}

impl<K, V> Merge for HashMap<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Merge + Clone,
{
    fn merge(mut self, other: Self) -> Self {
        for (key, other_value) in other {
            if let Some(self_value) = self.remove(&key) {
                self.insert(key, self_value.merge(other_value));
            } else {
                self.insert(key, other_value);
            }
        }
        self
    }
}

impl<K, V> Merge for BTreeMap<K, V>
where
    K: Ord + Clone,
    V: Merge + Clone,
{
    fn merge(mut self, other: Self) -> Self {
        for (key, other_value) in other {
            if let Some(self_value) = self.remove(&key) {
                self.insert(key, self_value.merge(other_value));
            } else {
                self.insert(key, other_value);
            }
        }
        self
    }
}

/// Vec merging: complete replacement (other wins entirely).
///
/// This matches Nickel's behavior where array fields are replaced, not merged.
/// For element-wise merging, use a custom implementation.
impl<T> Merge for Vec<T> {
    fn merge(self, other: Self) -> Self {
        if other.is_empty() {
            self
        } else {
            other
        }
    }
}

// Primitive types: other wins
impl Merge for String {
    fn merge(self, other: Self) -> Self {
        if other.is_empty() {
            self
        } else {
            other
        }
    }
}

impl Merge for bool {
    fn merge(self, other: Self) -> Self {
        other
    }
}

impl Merge for i32 {
    fn merge(self, other: Self) -> Self {
        other
    }
}

impl Merge for i64 {
    fn merge(self, other: Self) -> Self {
        other
    }
}

impl Merge for u32 {
    fn merge(self, other: Self) -> Self {
        other
    }
}

impl Merge for u64 {
    fn merge(self, other: Self) -> Self {
        other
    }
}

impl Merge for f32 {
    fn merge(self, other: Self) -> Self {
        other
    }
}

impl Merge for f64 {
    fn merge(self, other: Self) -> Self {
        other
    }
}

// serde_json::Value merge
impl Merge for serde_json::Value {
    fn merge(self, other: Self) -> Self {
        use serde_json::Value;

        match (self, other) {
            // Both objects: deep merge
            (Value::Object(mut base), Value::Object(overlay)) => {
                for (key, overlay_value) in overlay {
                    if let Some(base_value) = base.remove(&key) {
                        base.insert(key, base_value.merge(overlay_value));
                    } else {
                        base.insert(key, overlay_value);
                    }
                }
                Value::Object(base)
            }
            // Both arrays: replace (could also concatenate, but replacement is more common)
            (Value::Array(_), Value::Array(overlay)) => Value::Array(overlay),
            // Different types or primitives: overlay wins
            (_, overlay) => overlay,
        }
    }
}

/// Extension trait for merging Option fields with defaults.
pub trait OptionMergeExt<T> {
    /// Get or insert the default value, returning a mutable reference.
    fn get_or_insert_default(&mut self) -> &mut T
    where
        T: Default;

    /// Merge the inner value if both are Some, otherwise use the available one.
    fn merge_option(self, other: Self) -> Self
    where
        T: Merge;
}

impl<T> OptionMergeExt<T> for Option<T> {
    fn get_or_insert_default(&mut self) -> &mut T
    where
        T: Default,
    {
        self.get_or_insert_with(T::default)
    }

    fn merge_option(self, other: Self) -> Self
    where
        T: Merge,
    {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.merge(b)),
            (a, b) => b.or(a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Inner {
        value: Option<i32>,
        name: Option<String>,
    }

    impl Merge for Inner {
        fn merge(self, other: Self) -> Self {
            Self {
                value: other.value.or(self.value),
                name: other.name.or(self.name),
            }
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Outer {
        inner: Option<Inner>,
        count: Option<i32>,
    }

    impl Merge for Outer {
        fn merge(self, other: Self) -> Self {
            Self {
                inner: self.inner.merge(other.inner),
                count: other.count.or(self.count),
            }
        }
    }

    #[test]
    fn test_nested_merge() {
        let base = Outer {
            inner: Some(Inner {
                value: Some(1),
                name: Some("base".into()),
            }),
            count: Some(10),
        };

        let overlay = Outer {
            inner: Some(Inner {
                value: Some(2),
                name: None, // Keep base
            }),
            count: None, // Keep base
        };

        let merged = base.merge(overlay);

        assert_eq!(merged.inner.as_ref().unwrap().value, Some(2)); // Overlay
        assert_eq!(
            merged.inner.as_ref().unwrap().name,
            Some("base".into())
        ); // Base preserved
        assert_eq!(merged.count, Some(10)); // Base preserved
    }

    #[test]
    fn test_merge_with() {
        let config = Outer::default()
            .merge_with(|c| c.count = Some(5))
            .merge_with(|c| {
                c.inner = Some(Inner {
                    value: Some(42),
                    name: None,
                })
            });

        assert_eq!(config.count, Some(5));
        assert_eq!(config.inner.unwrap().value, Some(42));
    }

    #[test]
    fn test_merge_all() {
        let base = Outer {
            count: Some(1),
            inner: None,
        };

        let overlays = vec![
            Outer {
                count: Some(2),
                inner: None,
            },
            Outer {
                count: Some(3),
                inner: None,
            },
        ];

        let merged = base.merge_all(overlays);
        assert_eq!(merged.count, Some(3)); // Last overlay wins
    }

    #[test]
    fn test_hashmap_merge() {
        let mut base = HashMap::new();
        base.insert("a".to_string(), Some(1));
        base.insert("b".to_string(), Some(2));

        let mut overlay = HashMap::new();
        overlay.insert("b".to_string(), Some(20)); // Override
        overlay.insert("c".to_string(), Some(3)); // New key

        let merged = base.merge(overlay);

        assert_eq!(merged.get("a"), Some(&Some(1))); // Preserved
        assert_eq!(merged.get("b"), Some(&Some(20))); // Overridden
        assert_eq!(merged.get("c"), Some(&Some(3))); // New
    }

    #[test]
    fn test_json_value_merge() {
        use serde_json::json;

        let base = json!({
            "name": "app",
            "spec": {
                "replicas": 1,
                "image": "base:v1"
            }
        });

        let overlay = json!({
            "spec": {
                "replicas": 3
            },
            "labels": {
                "env": "prod"
            }
        });

        let merged = base.merge(overlay);

        assert_eq!(merged["name"], "app"); // Preserved
        assert_eq!(merged["spec"]["replicas"], 3); // Overridden
        assert_eq!(merged["spec"]["image"], "base:v1"); // Preserved
        assert_eq!(merged["labels"]["env"], "prod"); // New
    }

    #[test]
    fn test_vec_merge_replacement() {
        let base = vec![1, 2, 3];
        let overlay = vec![4, 5];

        let merged = base.merge(overlay);
        assert_eq!(merged, vec![4, 5]); // Complete replacement
    }

    #[test]
    fn test_vec_merge_empty_overlay() {
        let base = vec![1, 2, 3];
        let overlay: Vec<i32> = vec![];

        let merged = base.merge(overlay);
        assert_eq!(merged, vec![1, 2, 3]); // Base preserved when overlay empty
    }
}
