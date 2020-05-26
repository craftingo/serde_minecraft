//! Abstractions over the `byte` type used within Minecraft protocol packets.

use std::ops::{Deref, DerefMut};

/// The `byte` type utilised by the Minecraft protocol.
///
/// Bytes are signed 8-bit integers represented through the [two's complement]
/// operation.
///
/// See [the protocol documentation] for further details.
///
/// [two's complement]]: https://en.wikipedia.org/wiki/Two%27s_complement
/// [the protocol documentation]: https://wiki.vg/Protocol#Data_types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Byte(i8);

impl Byte {
    /// Creates a new [`Byte`] from its value.
    ///
    /// [`Byte`]: struct.Byte.html
    pub fn new(byte: i8) -> Self {
        Byte(byte)
    }
}

impl From<i8> for Byte {
    fn from(value: i8) -> Self {
        Self::new(value)
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self::new(value as i8)
    }
}

impl Into<i8> for Byte {
    fn into(self) -> i8 {
        self.0
    }
}

impl Deref for Byte {
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Byte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
