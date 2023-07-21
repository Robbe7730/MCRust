use std::convert::TryInto;

use crate::packets::packet_writer::PacketWriter;
use crate::server::CommandNode;

use super::Clientbound;

#[derive(Debug, Clone)]
pub struct DeclareCommandsPacket {
    pub nodes: Vec<CommandNode>,
    pub root_node: i32,
}

impl Clientbound for DeclareCommandsPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x10);

        writer.add_varint(self.nodes.len().try_into().unwrap());

        for node in self.nodes.iter() {
            node.write(&mut writer);
        }

        writer.add_varint(self.root_node);

        writer
    }
}
