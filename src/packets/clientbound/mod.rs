pub mod chat_message;
pub mod held_item_change;
pub mod join_game;
pub mod keep_alive;
pub mod legacy_ping;
pub mod login_success;
pub mod player_position_and_look;
pub mod pong;
pub mod status_response;
pub mod chunk_data;
pub mod update_view_position;
pub mod plugin_message;
pub mod change_difficulty;
pub mod player_abilities;
pub mod declare_recipes;
pub mod unlock_recipes;
pub mod tags;
pub mod entity_status;

pub use chat_message::*;
pub use held_item_change::*;
pub use join_game::*;
pub use keep_alive::*;
pub use legacy_ping::*;
pub use login_success::*;
pub use player_position_and_look::*;
pub use pong::*;
pub use status_response::*;
pub use chunk_data::*;
pub use update_view_position::*;
pub use plugin_message::*;
pub use change_difficulty::*;
pub use player_abilities::*;
pub use declare_recipes::*;
pub use unlock_recipes::*;
pub use tags::*;
pub use entity_status::*;

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
    ChunkData(ChunkDataPacket),
    UpdateViewPosition(UpdateViewPositionPacket),
    PluginMessage(PluginMessagePacket),
    ChangeDifficulty(ChangeDifficultyPacket),
    PlayerAbilities(PlayerAbilitiesPacket),
    DeclareRecipes(DeclareRecipesPacket),
    UnlockRecipes(UnlockRecipesPacket),
    Tags(TagsPacket),
    EntityStatus(EntityStatusPacket),
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
            ClientboundPacket::ChunkData(p) => p.writer(),
            ClientboundPacket::UpdateViewPosition(p) => p.writer(),
            ClientboundPacket::PluginMessage(p) => p.writer(),
            ClientboundPacket::ChangeDifficulty(p) => p.writer(),
            ClientboundPacket::PlayerAbilities(p) => p.writer(),
            ClientboundPacket::DeclareRecipes(p) => p.writer(),
            ClientboundPacket::UnlockRecipes(p) => p.writer(),
            ClientboundPacket::Tags(p) => p.writer(),
            ClientboundPacket::EntityStatus(p) => p.writer(),
        }
    }
}
