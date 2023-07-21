use std::time::Instant;

use rand::random;

use crate::player::Player;

use super::super::Clientbound;
use super::super::packet_writer::PacketWriter;

#[derive(Debug, Clone)]
pub struct KeepAlivePacket {
    id: i64,
}

impl Clientbound for KeepAlivePacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x1F);
        writer.add_signed_long(self.id);
        writer
    }
}

impl KeepAlivePacket {
    pub fn for_player(player: &mut Player) -> Self {
        let id = random();
        player.last_keepalive_sent = Some((id, Instant::now()));
        Self {
            id
        }
    }
}
