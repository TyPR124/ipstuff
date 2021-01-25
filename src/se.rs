//! Serialization helpers

use std::fmt::Display;

use serde::{Serialize, Serializer};

/// Serializes the provided item using alternate [`Display`] formatting.
///
/// The default serialization behavior is often to use the standard [`Display`] formatting.
/// Using this function for serialization will cause the type to be serialized with alternate [`Display`] formatting.
///
/// Note that this function has no effect when serializing to a non human readable format, such as [`bincode`]. That is,
/// this function changes the textual serialized representation of an item, but does not change its binary serialized representation.
pub fn alternate<T: Display + Serialize, S: Serializer>(
    item: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if serializer.is_human_readable() {
        serializer.collect_str(&format_args!("{:#}", item))
    } else {
        item.serialize(serializer)
    }
}
