use crate::packets::Packet;
use crate::packet_reader::PacketReader;

#[macro_export]
macro_rules! expect_equal {
    ( $actual:expr, $expected:expr ) => {
        {
            if ($actual != $expected) {
                Err(format!("Expedted {} but got {}", $expected, $actual))?
            } else {
                $expected
            }
        }
    };
}

pub struct LegacyPingServerbound {
    hostname: String,
    port: u32,
}

impl Packet for LegacyPingServerbound {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, String> {
        expect_equal!(reader.read_unsigned_byte().map_err(|x| x.to_string())?, 0xfe);
        expect_equal!(reader.read_unsigned_byte().map_err(|x| x.to_string())?, 0x01);
        expect_equal!(reader.read_unsigned_byte().map_err(|x| x.to_string())?, 0xfa);
        expect_equal!(reader.read_unsigned_short().map_err(|x| x.to_string())?, 0x000b);
        expect_equal!(reader.read_string(11).map_err(|x| x.to_string())?, "MC|PingHost");
        let _remaining_data = reader.read_unsigned_short()?;
        let _protocol_version = reader.read_unsigned_byte()?;
        let hostname_len = reader.read_unsigned_short()?;
        let hostname = reader.read_string(hostname_len.into())?;
        let port = reader.read_unsigned_int()?;
        println!("PING @ {}:{}", hostname, port);
        Ok(Self {
            hostname: hostname,
            port: port,
        })
    }
}
