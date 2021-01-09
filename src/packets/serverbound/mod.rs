pub mod chat_message;
pub mod client_settings;
pub mod handshaking;
pub mod keep_alive;
pub mod legacy_ping;
pub mod login_start;
pub mod ping;
pub mod status_request;

pub use chat_message::*;
pub use client_settings::*;
pub use handshaking::*;
pub use keep_alive::*;
pub use legacy_ping::*;
pub use login_start::*;
pub use ping::*;
pub use status_request::*;

use super::packet_reader::PacketReader;

use crate::error_type::ErrorType;

#[derive(Debug)]
pub enum ServerboundPacket {
    LegacyPing(LegacyPingServerboundPacket),
    Handshaking(HandshakingPacket),
    StatusRequest(StatusRequestPacket),
    Ping(PingPacket),
    LoginStart(LoginStartPacket),
    ClientSettings(ClientSettingsPacket),
    ChatMessage(ChatMessagePacket),
    KeepAlive(KeepAlivePacket),
}

pub trait Serverbound {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType>
    where
        Self: Sized;
}
