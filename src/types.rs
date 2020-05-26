use displaydoc::Display;
use std::io::{self, Read, Write};
use thiserror::Error;

#[allow(unused)]
macro_rules! var_impl {
    ($impl:ident, $max:expr, $type:path) => {
        impl $impl {
            /// Creates a new VarInt that holds the given value.
            pub fn new(val: $type) -> Self {
                Self { val }
            }

            /// Tries to parse a VarInt from the given input.
            pub fn read(read: &mut dyn Read) -> Result<Self, ReadError> {
                let mut buf = [0; $max];
                read.read(&mut buf)?;
                let mut num_read = 0usize;
                let mut result = 0;

                loop {
                    let byte = buf[num_read];
                    let val = byte & 0x7F;

                    result |= (val as $type) << (7 * num_read);
                    num_read += 1;

                    if num_read > $max {
                        return Err(ReadError::TooBig);
                    }

                    if (byte & 0x80) == 0 {
                        break;
                    }
                }

                Ok(Self::new(result))
            }

            /// Writes the VarInt to the output.
            ///
            /// Returns the amount of bytes written.
            pub fn write(&self, write: &mut impl Write) -> Result<usize, io::Error> {
                let mut val = self.val;
                let mut buf = Vec::new();

                loop {
                    let mut temp = (val & 0x7F) as u8;
                    val >>= 7;
                    if val != 0 {
                        temp |= 0x80;
                    }
                    buf.push(temp);

                    if val == 0 {
                        break;
                    }
                }

                write.write(&buf)
            }

            /// Serializes this VarInt / VarLong and returns the raw bytes.
            pub fn write_to_vec(&self) -> Vec<u8> {
                let mut buf = Vec::new();
                self.write(&mut buf)
                    .expect("Write to Vec should never fail.");
                buf
            }
        }

        impl From<$type> for $impl {
            fn from(val: $type) -> Self {
                Self::new(val)
            }
        }

        impl ::std::ops::Deref for $impl {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.val
            }
        }

        impl ::std::ops::DerefMut for $impl {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.val
            }
        }
    };
}

/// Indicates that the var long / int is too big and thus can't
/// be parsed.
#[derive(Debug, Error, Display)]
pub enum ReadError {
    /// The VarInt / VarLong is too big.
    TooBig,
    /// I/O error occured: {0}.
    Io(#[from] io::Error),
}

/// VarInt datatype as described by the [`Minecraft Protocol`].
///
/// [`Minecraft Protocol`]: https://wiki.vg/Protocol
#[derive(Debug)]
pub struct VarInt {
    /// The value of this VarInt, in an unsigned 32bit integer.
    pub val: i32,
}
var_impl!(VarInt, 5, i32);

/// VarLong datatype as described by the [`Minecraft Protocol`].
///
/// [`Minecraft Protocol`]: https://wiki.vg/Protocol
#[derive(Debug)]
pub struct VarLong {
    /// The value of this VarInt, in an unsigned 32bit integer.
    pub val: i64,
}
var_impl!(VarLong, 10, i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let mut val: VarInt = 1337.into();
        assert_eq!(1337, val.val);
        *val += 10;
        assert_eq!(1347, val.val);
    }

    #[test]
    fn test_write_and_read() {
        let val: VarInt = 12345.into();
        let mut buf = Vec::new();

        assert!(val.write(&mut buf).is_ok());

        let new_val = VarInt::read(&mut buf.as_slice());

        assert!(new_val.is_ok());
        assert_eq!(val.val, new_val.unwrap().val);
    }
}
