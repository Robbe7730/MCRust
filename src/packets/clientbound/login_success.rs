use crate::packets::Clientbound;
use crate::packets::packet_writer::PacketWriter;

use uuid::Uuid;

#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
}

impl Clientbound for LoginSuccessPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x02);
        writer.add_uuid(self.uuid);
        writer.add_string(&self.username);
        writer
    }
}

impl LoginSuccessPacket {
    pub fn new(username: String) -> Self {
        let username_bytes = format!("OfflinePlayer:{}", username).bytes().collect::<Vec<u8>>();
        LoginSuccessPacket {
            uuid: Uuid::new_v3(&Uuid::NAMESPACE_URL, &username_bytes),
            username: username,
        }
    }
}
