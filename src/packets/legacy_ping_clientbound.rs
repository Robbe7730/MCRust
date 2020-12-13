use crate::packets::Clientbound;
use crate::packet_writer::PacketWriter;

pub struct LegacyPingClientboundPacket {
    pub protocol_version: usize,
    pub minecraft_version: String,
    pub motd: String,
    // TODO: figure out if this can be any string
    pub curr_player_count: usize,
    pub max_player_count: usize,
}

impl Clientbound for LegacyPingClientboundPacket {
    fn writer(&self) -> PacketWriter {
        let protocol_version_string = format!("{}", self.protocol_version);
        let curr_player_count_string = format!("{}", self.curr_player_count);
        let max_player_count_string = format!("{}", self.max_player_count);
        let len: u16 = (7 +
            protocol_version_string.len() +
            self.minecraft_version.len() + 
            self.motd.len() + 
            curr_player_count_string.len() +
            max_player_count_string.len()) as u16;
        let mut ret = PacketWriter::new();
        ret.add_unsigned_byte(0xff); // Kick packet
        ret.add_unsigned_short(len); // Length of remaining string
        ret.add_string_null_terminated(&"§1".to_string());
        ret.add_string_null_terminated(&protocol_version_string);
        ret.add_string_null_terminated(&self.minecraft_version);
        ret.add_string_null_terminated(&self.motd);
        ret.add_string_null_terminated(&curr_player_count_string);
        ret.add_string(&max_player_count_string);
        ret
    }
}

