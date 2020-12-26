use crate::packets::packet_writer::PacketWriter;
use crate::packets::Clientbound;

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
