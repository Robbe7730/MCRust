pub mod join_game;
pub mod legacy_ping;
pub mod login_success;
pub mod pong;
pub mod status_response;
pub mod held_item_change;
pub mod player_position_and_look;
pub mod chat_message;

pub use join_game::*;
pub use legacy_ping::*;
pub use login_success::*;
pub use pong::*;
pub use status_response::*;
pub use held_item_change::*;
pub use player_position_and_look::*;
pub use chat_message::*;

use super::packet_writer::PacketWriter;

#[derive(Debug)]
pub enum ClientboundPacket {
    LegacyPing(LegacyPingClientboundPacket),
    StatusResponse(StatusResponsePacket),
    Pong(PongPacket),
    LoginSuccess(LoginSuccessPacket),
    JoinGame(JoinGamePacket),
    HeldItemChange(HeldItemChangePacket),
    PlayerPositionAndLook(PlayerPositionAndLookPacket),
    ChatMessage(ChatMessagePacket),
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
        }
    }
}
