use rand::random;

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
use crate::Eid;
use crate::Server;

use std::convert::TryInto;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct PlayState {
    player_eid: Eid,
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
                let entity = entity_arc.read().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for reading: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player()?;

                // Send Held Item
                queue.push(ClientboundPacket::HeldItemChange(HeldItemChangePacket::from_player(&player)));

                // Send available recipies
                queue.push(ClientboundPacket::DeclareRecipies(DeclareRecipiesPacket {
                    recipies: server_lock.recipies.clone(),
                }));

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
