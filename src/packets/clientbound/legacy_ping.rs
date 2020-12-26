use crate::packets::packet_writer::PacketWriter;
use crate::packets::Clientbound;

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
        let len: u16 = (7
            + protocol_version_string.len()
            + self.minecraft_version.len()
            + self.motd.len()
            + curr_player_count_string.len()
            + max_player_count_string.len()) as u16;
        let mut ret = PacketWriter::new_legacy(0xff);
        ret.add_unsigned_short(len); // Length of remaining string
        ret.add_utf16_string(&"§1".to_string());
        ret.add_unsigned_short(0x0000);
        ret.add_utf16_string(&protocol_version_string);
        ret.add_unsigned_short(0x0000);
        ret.add_utf16_string(&self.minecraft_version);
        ret.add_unsigned_short(0x0000);
        ret.add_utf16_string(&self.motd);
        ret.add_unsigned_short(0x0000);
        ret.add_utf16_string(&curr_player_count_string);
        ret.add_unsigned_short(0x0000);
        ret.add_utf16_string(&max_player_count_string);
        ret
    }
}
