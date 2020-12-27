pub mod handshaking;
pub mod legacy_ping;
pub mod login_start;
pub mod ping;
pub mod status_request;

pub use handshaking::*;
pub use legacy_ping::*;
pub use login_start::*;
pub use ping::*;
pub use status_request::*;

use super::packet_reader::PacketReader;

use crate::error_type::ErrorType;

pub enum ServerboundPacket {
    LegacyPing(LegacyPingServerboundPacket),
    Handshaking(HandshakingPacket),
    StatusRequest(StatusRequestPacket),
    Ping(PingPacket),
    LoginStart(LoginStartPacket),
}

pub trait Serverbound {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType>
    where
        Self: Sized;
}
