use crate::packets::Serverbound;

use crate::expect_equal;
use crate::packet_reader::PacketReader;

pub struct LegacyPingServerboundPacket {
    pub hostname: String,
    pub port: u32,
}

impl Serverbound for LegacyPingServerboundPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, String> {
        expect_equal!(reader.read_unsigned_byte().map_err(|x| x.to_string())?, 0xfe);
        expect_equal!(reader.read_unsigned_byte().map_err(|x| x.to_string())?, 0x01);
        expect_equal!(reader.read_unsigned_byte().map_err(|x| x.to_string())?, 0xfa);
        expect_equal!(reader.read_unsigned_short().map_err(|x| x.to_string())?, 0x000b);
        expect_equal!(reader.read_string_chars(11).map_err(|x| x.to_string())?, "MC|PingHost");
        let _remaining_data = reader.read_unsigned_short()?;
        let _protocol_version = reader.read_unsigned_byte()?;
        let hostname_len = reader.read_unsigned_short()?;
        let hostname = reader.read_string_chars(hostname_len.into())?;
        let port = reader.read_unsigned_int()?;
        Ok(Self {
            hostname: hostname,
            port: port,
        })
    }
}
