pub mod chat_message;
pub mod client_settings;
pub mod handshaking;
pub mod keep_alive;
pub mod legacy_ping;
pub mod login_start;
pub mod ping;
pub mod status_request;
pub mod plugin_message;
pub mod teleport_confirm;
pub mod player_position_and_rotation;
pub mod held_item_change;
pub mod set_recipe_book_state;

pub use chat_message::*;
pub use client_settings::*;
pub use handshaking::*;
pub use keep_alive::*;
pub use legacy_ping::*;
pub use login_start::*;
pub use ping::*;
pub use status_request::*;
pub use plugin_message::*;
pub use teleport_confirm::*;
pub use player_position_and_rotation::*;
pub use held_item_change::*;
pub use set_recipe_book_state::*;

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
    PluginMessage(PluginMessagePacket),
    TeleportConfirm(TeleportConfirmPacket),
    PlayerPositionAndRotation(PlayerPositionAndRotationPacket),
    HeldItemChange(HeldItemChangePacket),
    SetRecipeBookState(SetRecipeBookStatePacket),
}

pub trait Serverbound {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType>
    where
        Self: Sized;
}
