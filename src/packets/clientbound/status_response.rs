use super::super::packet_writer::PacketWriter;
use super::super::Clientbound;

use crate::chat::Chat;
use crate::player::Player;

use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug, Clone)]
pub struct StatusResponsePlayer {
    pub name: String,
    pub id: String,
}

impl From<&Player> for StatusResponsePlayer {
    fn from(player: &Player) -> Self {
        Self {
            name: player.username.clone(),
            id: player.uuid.to_hyphenated().to_string(),
        }
    }
}

//TODO favicon
#[derive(Debug, Clone)]
pub struct StatusResponsePacket {
    pub version_name: String,
    pub version_protocol: usize,
    pub players_max: usize,
    pub players_curr: usize,
    pub sample: Vec<StatusResponsePlayer>,
    pub description: Chat,
}

impl Clientbound for StatusResponsePacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x00);
        let status_json = json!({
            "version": {
                "name": self.version_name,
                "protocol": self.version_protocol,
            },
            "players": {
                "max": self.players_max,
                "online": self.players_curr,
                "sample": self.sample,
            },
            "description": self.description,
        });
        writer.add_json(status_json);
        writer
    }
}
