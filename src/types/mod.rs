//! Type abstractions over primitives that are used in the Minecraft protocol packets.

use std::io::{Read, Result, Write};

pub use self::bool::Bool;
pub use self::byte::Byte;

/// A trait for reading and parsing Minecraft protocol types from an underlying source
/// of serialized binary data.
///
/// Readers, so-called implementors of this trait, are composed of multiple methods which
/// allow for reading and parsing effectively any supported primitive used by the protocol.
///
/// Readers are meant to be the primary layer for directly processing data received from an
/// arbitrary input source to ensure integrity of the data into applications in a safe and
/// strictly typed manner.
pub trait PacketReader: Read {
    /// Reads a [`Bool`] from an underlying source of data.
    ///
    /// [`Bool`]: struct.Bool.html
    fn read_bool(&mut self) -> Result<Bool>;

    /// Reads a [`Byte`] from an underlying source of data.
    ///
    /// [`Byte`]: struct.Byte.html
    fn read_byte(&mut self) -> Result<Byte>;
}

/// A trait for parsing and writing Rust abstractions over the Minecraft protocol types to
/// an underlying source of serialized binary data.
///
/// Writers, so-called implementors of this trait, are composed of multiple methods which
/// allow for parsing and writing any supported primitive by the protocol into compatible
/// binary data.
///
/// Writers are meant to be the layer for directly processing application data into
/// serialized and correct binary data which can be transferred over the network.
pub trait PacketWriter: Write {
    /// Writes a [`Bool`] to an underlying source of data.
    ///
    /// [`Bool`]: struct.Bool.html
    fn write_bool(&mut self, b: Bool) -> Result<()>;

    /// Writes a [`Byte`] to an underlying source of data.
    ///
    /// [`Byte`]: struct.Byte.html
    fn write_byte(&mut self, b: Byte) -> Result<()>;
}

mod bool;
mod byte;
