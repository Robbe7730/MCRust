pub mod chat_message;
pub mod held_item_change;
pub mod join_game;
pub mod keep_alive;
pub mod legacy_ping;
pub mod login_success;
pub mod player_position_and_look;
pub mod pong;
pub mod status_response;

pub use chat_message::*;
pub use held_item_change::*;
pub use join_game::*;
pub use keep_alive::*;
pub use legacy_ping::*;
pub use login_success::*;
pub use player_position_and_look::*;
pub use pong::*;
pub use status_response::*;

use super::packet_writer::PacketWriter;

#[derive(Debug, Clone)]
pub enum ClientboundPacket {
    LegacyPing(LegacyPingClientboundPacket),
    StatusResponse(StatusResponsePacket),
    Pong(PongPacket),
    LoginSuccess(LoginSuccessPacket),
    JoinGame(JoinGamePacket),
    HeldItemChange(HeldItemChangePacket),
    PlayerPositionAndLook(PlayerPositionAndLookPacket),
    ChatMessage(ChatMessagePacket),
    KeepAlive(KeepAlivePacket),
}

pub trait Clientbound {
    fn writer(&self) -> PacketWriter;
}

impl Clientbound for ClientboundPacket {
    fn writer(&self) -> PacketWriter {
        match self {
            ClientboundPacket::LegacyPing(p) => p.writer(),
            ClientboundPacket::StatusResponse(p) => p.writer(),
            ClientboundPacket::Pong(p) => p.writer(),
            ClientboundPacket::LoginSuccess(p) => p.writer(),
            ClientboundPacket::JoinGame(p) => p.writer(),
            ClientboundPacket::HeldItemChange(p) => p.writer(),
            ClientboundPacket::PlayerPositionAndLook(p) => p.writer(),
            ClientboundPacket::ChatMessage(p) => p.writer(),
            ClientboundPacket::KeepAlive(p) => p.writer(),
        }
    }
}
