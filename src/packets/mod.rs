pub mod legacy_ping;
pub use legacy_ping::*;

use crate::packet_reader::PacketReader;

pub trait Packet {
    fn from_reader(stream: &mut PacketReader) -> Result<Self, String>
        where Self: Sized;
}
