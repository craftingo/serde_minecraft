//! Abstractions over the `bool` type used within Minecraft protocol packets.

use std::ops::{Deref, DerefMut};

fn decode_value(value: u8) -> bool {
    // The protocol actually expects `true` to be strictly bound to the
    // value of `1`, but we're generous here and allow any non-null value.
    value != 0
}

fn encode_value(value: bool) -> u8 {
    match value {
        true => 1,
        false => 0,
    }
}

/// The `bool` type utilised by the Minecraft protocol.
///
/// Bytes are unsigned 8-bit integers, holding possible two values:
///
/// - `1` for `true`
/// - `0` for `false`
///
/// See [the protocol documentation] for further details.
///
/// [the protocol documentation]: https://wiki.vg/Protocol#Data_types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bool(bool);

impl Bool {
    /// Creates a new [`Bool`] from its value.
    ///
    /// [`Bool`]: struct.Bool.html
    pub fn new(value: bool) -> Self {
        Bool(value)
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Bool::new(value)
    }
}

impl From<u8> for Bool {
    fn from(value: u8) -> Self {
        Bool::new(decode_value(value))
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        self.0
    }
}

impl Into<u8> for Bool {
    fn into(self) -> u8 {
        encode_value(self.0)
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
