use std::convert::TryInto;

use serde_json::json;
use uuid::Uuid;

use super::Clientbound;
use crate::packets::packet_writer::PacketWriter;
use crate::player::{Gamemode, Player};
use crate::chat::Chat;

#[derive(Debug, Clone)]
pub struct PlayerInfoProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>
}

#[derive(Debug, Clone)]
pub enum PlayerInfoPacket {
    // UUID, Properties, Gamemode, Ping, Display name
    AddPlayer(Vec<(Uuid, String, Vec<PlayerInfoProperty>, Gamemode, i32, Option<Chat>)>),

    // UUID, Gamemode
    UpdateGamemode(Vec<(Uuid, Gamemode)>),

    // UUID, Ping
    UpdateLatency(Vec<(Uuid, i32)>),

    // UUID, Display Name
    UpdateDisplayName(Vec<(Uuid, Option<Chat>)>),

    // UUID
    RemovePlayer(Vec<Uuid>)
}

impl Clientbound for PlayerInfoPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x32);

        let action_id = match self {
            PlayerInfoPacket::AddPlayer(_) => 0,
            PlayerInfoPacket::UpdateGamemode(_) => 1,
            PlayerInfoPacket::UpdateLatency(_) => 2,
            PlayerInfoPacket::UpdateDisplayName(_) => 3,
            PlayerInfoPacket::RemovePlayer(_) => 4
        };

        writer.add_varint(action_id);

        let num_players = match self {
            PlayerInfoPacket::AddPlayer(v) => v.len(),
            PlayerInfoPacket::UpdateGamemode(v) => v.len(),
            PlayerInfoPacket::UpdateLatency(v) => v.len(),
            PlayerInfoPacket::UpdateDisplayName(v) => v.len(),
            PlayerInfoPacket::RemovePlayer(v) => v.len()
        };

        writer.add_varint(num_players.try_into().unwrap());

        match self {
            PlayerInfoPacket::AddPlayer(players) => {
                for (uuid, name, properties, gamemode, ping, maybe_displayname) in players {
                    writer.add_uuid(*uuid);
                    writer.add_string(name);
                    writer.add_varint(properties.len().try_into().unwrap());
                    for property in properties {
                        writer.add_string(&property.name);
                        writer.add_string(&property.value);
                        writer.add_boolean(property.signature.is_some());
                        if let Some(signature) = property.signature.as_ref() {
                            writer.add_string(&signature);
                        }
                    }

                    writer.add_varint(*gamemode as i32);
                    writer.add_varint(*ping);
                    writer.add_boolean(maybe_displayname.is_some());

                    if let Some(displayname) = maybe_displayname {
                        writer.add_json(json!(displayname));
                    }
                }
            }
            PlayerInfoPacket::UpdateGamemode(players) => {
                for (uuid, gamemode) in players {
                    writer.add_uuid(*uuid);
                    writer.add_varint(*gamemode as i32);
                }
            }
            PlayerInfoPacket::UpdateLatency(players) => {
                for (uuid, latency) in players {
                    writer.add_uuid(*uuid);
                    writer.add_varint(*latency);
                }
            }
            PlayerInfoPacket::UpdateDisplayName(players) => {
                for (uuid, maybe_displayname) in players {
                    writer.add_uuid(*uuid);
                    writer.add_boolean(maybe_displayname.is_some());
                    if let Some(displayname) = maybe_displayname {
                        writer.add_json(json!(displayname));
                    }
                }
            }
            PlayerInfoPacket::RemovePlayer(players) => {
                for uuid in players {
                    writer.add_uuid(*uuid)
                }
            }
        }

        writer
    }
}

impl PlayerInfoPacket {
    pub fn add_players(players: Vec<&Player>) -> Self {
        let mut data = vec![];

        for player in players {
            let mut player_properties = vec![];
            for (property_name, (value, signature)) in player.properties.iter() {
                player_properties.push(PlayerInfoProperty {
                    name: property_name.to_owned(),
                    value: value.to_owned(),
                    signature: signature.to_owned()
                });
            }
            data.push((
                player.uuid,
                player.username.to_owned(),
                player_properties,
                player.gamemode,
                player.latency.unwrap_or(0), // Defaulting to 0 if no latency is measured
                player.displayname.to_owned()
            ))
        }

        PlayerInfoPacket::AddPlayer(data)
    }
}
