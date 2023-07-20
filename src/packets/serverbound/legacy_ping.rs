use super::super::packet_reader::PacketReader;
use super::super::Serverbound;

use crate::error_type::ErrorType;
use crate::expect_equal;

#[derive(Debug)]
pub struct LegacyPingServerboundPacket {
    pub hostname: String,
    pub port: u32,
}

impl Serverbound for LegacyPingServerboundPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        expect_equal!(reader.read_unsigned_byte()?, 0xfe);
        expect_equal!(reader.read_unsigned_byte()?, 0x01);
        expect_equal!(reader.read_unsigned_byte()?, 0xfa);
        expect_equal!(reader.read_unsigned_short()?, 0x000b);
        expect_equal!(reader.read_string_chars(11)?, "MC|PingHost");
        let _remaining_data = reader.read_unsigned_short()?;
        let _protocol_version = reader.read_unsigned_byte()?;
        let hostname_len = reader.read_unsigned_short()?;
        let hostname = reader.read_string_chars(hostname_len.into())?;
        let port = reader.read_unsigned_int()?;
        Ok(Self {
            hostname,
            port,
        })
    }
}
