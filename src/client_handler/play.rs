use rand::random;
use uuid::Uuid;

use super::ConnectionState;
use super::ConnectionStateTrait;
use super::ConnectionStateTransition;

use crate::chat::Chat;
use crate::chat::ChatPosition;
use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::nbt::NBTReader;
use crate::packets::serverbound::RecipeBook;
use crate::packets::serverbound::ServerboundPacket;
use crate::Server;
use crate::player::OPLevel;

use std::convert::TryInto;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct PlayState {
    player_eid: i32,
}

impl ConnectionStateTrait for PlayState {
    fn from_state(prev_state: ConnectionState) -> Result<Self, ErrorType> {
        match prev_state {
            ConnectionState::Login(login_state) => Ok(Self {
                player_eid: login_state.player_eid,
            }),
            x => Err(ErrorType::Fatal(format!(
                "Cannot go into Play state from {:#?}",
                x
            ))),
        }
    }

    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        server: Arc<Server>,
    ) -> Result<(Vec<ClientboundPacket>, ConnectionStateTransition), ErrorType> {
        let server_lock = server
            .data
            .lock()
            .map_err(|e| ErrorType::Fatal(format!("Could not lock server: {:?}", e)))?;
        let mut queue = vec![];
        match packet {
            ServerboundPacket::ClientSettings(_packet) => {
                // Get the player
                let world = server_lock.settings.worlds
                    .get(&server_lock.settings.selected_world)
                    .ok_or(ErrorType::Fatal("Invalid selected world".to_string()))?;

                let entity_arc = world
                    .get_entity(self.player_eid)?
                    .ok_or(ErrorType::Fatal("Player does not exist".to_string()))?;
                let mut entity = entity_arc.write().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for writing: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player_mut()?;

                // Send Held Item
                queue.push(ClientboundPacket::HeldItemChange(HeldItemChangePacket::from_player(&player)));

                // Send available recipes
                queue.push(ClientboundPacket::DeclareRecipes(DeclareRecipesPacket {
                    recipes: server_lock.recipes.clone(),
                }));

                // Send tags
                // queue.push(ClientboundPacket::Tags(TagsPacket::from_tags(&server_lock.tags)));

                // Send OP Level (Entity Status)
                queue.push(ClientboundPacket::EntityStatus(EntityStatusPacket{
                    entity_id: self.player_eid,
                    entity_status: match player.op_level {
                        OPLevel::Player => EntityStatus::PlayerOPLevel0,
                        OPLevel::Moderator => EntityStatus::PlayerOPLevel1,
                        OPLevel::Gamemaster => EntityStatus::PlayerOPLevel2,
                        OPLevel::Admin => EntityStatus::PlayerOPLevel3,
                        OPLevel::Owner => EntityStatus::PlayerOPLevel4,
                    }
                }));

                // TODO: Commands

                // TEMP: unlock all exisiting recipes
                player.unlocked_recipes = server_lock.recipes.iter().map(|r| r.id.clone()).collect();
                // Send unlocked recipes
                queue.push(ClientboundPacket::UnlockRecipes(UnlockRecipesPacket::init_from_player(&player)));

                // --v-- temporary, unordered packets for testing --v--

                // Send a welcome message
                queue.push(ClientboundPacket::ChatMessage(ChatMessagePacket {
                    message: Chat::new(format!(
                        "{} joined the game",
                        player.username
                    )),
                    sender: Uuid::nil(),
                    position: ChatPosition::SystemMessage,
                }));

                // Tell player where they are
                let player_chunk_x = (player.position.x / 16.0).floor() as i32;
                let player_chunk_z = (player.position.z / 16.0).floor() as i32;
                queue.push(ClientboundPacket::UpdateViewPosition(UpdateViewPositionPacket {
                    chunk_x: player_chunk_x,
                    chunk_z: player_chunk_z
                }));

                for x in -8i32..8 {
                    for z in -8i32..8 {
                        let column = world.get_chunk_column(
                            (player_chunk_x + x).try_into().unwrap(),
                            (player_chunk_z + z).try_into().unwrap(),
                        );
                        queue.push(ClientboundPacket::ChunkData(ChunkDataPacket::from_chunk_column(
                            x,
                            z,
                            column
                        )));
                    }
                }

                // Send the player position and look packet
                let x = player.position.x;
                let y = player.position.y;
                let z = player.position.z;
                let pitch = player.look.pitch;
                let yaw = player.look.yaw;
                queue.push(ClientboundPacket::PlayerPositionAndLook(PlayerPositionAndLookPacket {
                    x: ValueType::Absolute(x),
                    y: ValueType::Absolute(y),
                    z: ValueType::Absolute(z),
                    yaw: ValueType::Absolute(yaw),
                    pitch: ValueType::Absolute(pitch),
                    teleport_id: random(),
                }));

                Ok((queue, ConnectionStateTransition::Remain))
            }
            ServerboundPacket::ChatMessage(packet) => {
                let world = server_lock
                    .settings
                    .worlds
                    .get(&server_lock.settings.selected_world)
                    .ok_or(ErrorType::Fatal("Invalid selected".to_string()))?;

                // Get the player
                let entity_arc = world
                    .get_entity(self.player_eid)?
                    .ok_or(ErrorType::Fatal("Player does not exist".to_string()))?;
                let entity = entity_arc.read().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for reading: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player()?;

                // Send the message to all players
                let chat_packet = ClientboundPacket::ChatMessage(ChatMessagePacket {
                    message: Chat::new(format!("<{}> {}", player.username, packet.message)),
                    sender: player.uuid,
                    position: ChatPosition::SystemMessage,
                });
                server.send_to_all(chat_packet);
                Ok((queue, ConnectionStateTransition::Remain))
            }
            ServerboundPacket::KeepAlive(_packet) => {
                // TODO: validate response id == sent response id
                Ok((queue, ConnectionStateTransition::Remain))
            }
            ServerboundPacket::PluginMessage(packet) => {
                match packet.channel.as_str() {
                    "minecraft:brand" => {
                        println!(
                            "Player {} is using brand {}",
                            self.player_eid,
                            NBTReader::new(
                                packet.data.as_slice(),
                                packet.data.len().try_into().map_err(|_|
                                    ErrorType::Recoverable(format!("Packet data too big"))
                                )?
                            ).read_string()?
                        );
                        Ok((queue, ConnectionStateTransition::Remain))
                    }
                    c => {
                        Err(ErrorType::Recoverable(format!("Unimplemented channel {}" , c)))
                    }
                }
            }
            ServerboundPacket::TeleportConfirm(_packet) => {
                // TODO: validate teleport id == sent teleport id
                Ok((queue, ConnectionStateTransition::Remain))
            }
            ServerboundPacket::PlayerPositionAndRotation(packet) => {
                let world = server_lock
                    .settings
                    .worlds
                    .get(&server_lock.settings.selected_world)
                    .ok_or(ErrorType::Fatal("Invalid selected".to_string()))?;

                // Get the player
                let entity_arc = world
                    .get_entity(self.player_eid)?
                    .ok_or(ErrorType::Fatal("Player does not exist".to_string()))?;
                let mut entity = entity_arc.write().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for writing: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player_mut()?;

                player.position.x = packet.x;
                player.position.y = packet.feet_y;
                player.position.z = packet.z;
                player.look.yaw = packet.yaw;
                player.look.pitch = packet.pitch;
                player.position.on_ground = packet.on_ground;

                Ok((queue, ConnectionStateTransition::Remain))
            }
            ServerboundPacket::HeldItemChange(packet) => {
                if packet.slot > 8 || packet.slot < 0 {
                    return Err(ErrorType::Recoverable(format!(
                        "Invalid item slot {}",
                        packet.slot
                    )))
                }

                let world = server_lock
                    .settings
                    .worlds
                    .get(&server_lock.settings.selected_world)
                    .ok_or(ErrorType::Fatal("Invalid selected".to_string()))?;

                // Get the player
                let entity_arc = world
                    .get_entity(self.player_eid)?
                    .ok_or(ErrorType::Fatal("Player does not exist".to_string()))?;
                let mut entity = entity_arc.write().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for writing: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player_mut()?;

                player.selected_slot = packet.slot.try_into().unwrap();

                Ok((queue, ConnectionStateTransition::Remain))
            }
            ServerboundPacket::SetRecipeBookState(p) => {
                let world = server_lock
                    .settings
                    .worlds
                    .get(&server_lock.settings.selected_world)
                    .ok_or(ErrorType::Fatal("Invalid selected".to_string()))?;

                // Get the player
                let entity_arc = world
                    .get_entity(self.player_eid)?
                    .ok_or(ErrorType::Fatal("Player does not exist".to_string()))?;
                let mut entity = entity_arc.write().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for writing: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player_mut()?;

                match p.book_id {
                    RecipeBook::CraftingTable => {
                        player.recipe_book_state.crafting_table_open = p.book_open;
                        player.recipe_book_state.crafting_table_filter = p.filter_active;
                    }
                    RecipeBook::Furnace => {
                        player.recipe_book_state.furnace_open = p.book_open;
                        player.recipe_book_state.furnace_filter = p.filter_active;
                    }
                    RecipeBook::BlastFurnace => {
                        player.recipe_book_state.blast_furnace_open = p.book_open;
                        player.recipe_book_state.blast_furnace_filter = p.filter_active;
                    }
                    RecipeBook::Smoker => {
                        player.recipe_book_state.smoker_open = p.book_open;
                        player.recipe_book_state.smoker_filter = p.filter_active;
                    }
                }

                Ok((queue, ConnectionStateTransition::Remain))
            }
            x => Err(ErrorType::Recoverable(format!(
                "Unimplemented packet in Play state: {:#?}",
                x
            ))),
        }
    }
}
