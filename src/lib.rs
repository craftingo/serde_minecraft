//!
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms, missing_docs)]
#![warn(missing_docs)]

mod types;

pub use types::*;

use serde::{Deserialize, Serialize};

/// Represents a generic protocol Packet.
#[derive(Debug, Serialize, Deserialize)]
pub struct Packet<P: Serialize> {
    length: VarInt,
    packet_id: VarInt,
    #[serde(flatten)]
    data: P,
}
