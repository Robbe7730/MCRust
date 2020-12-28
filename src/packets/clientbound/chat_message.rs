use super::super::Clientbound;
use super::super::packet_writer::PacketWriter;

use crate::chat::Chat;
use crate::chat::ChatPosition;

use uuid::Uuid;
use serde_json::json;

pub struct ChatMessagePacket {
    pub message: Chat,
    pub position: ChatPosition,
    pub sender: Uuid,
}

impl Clientbound for ChatMessagePacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x0E);
        writer.add_json(json!(self.message));
        writer.add_signed_byte(self.position.clone().into());
        writer.add_uuid(self.sender);
        writer
    }
}
