pub mod handshaking;
pub mod legacy_ping;
pub mod ping;
pub mod status_request;
pub mod login_start;

pub use handshaking::*;
pub use legacy_ping::*;
pub use ping::*;
pub use status_request::*;
pub use login_start::*;

use crate::packet_reader::PacketReader;
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

