use super::serverbound::*;

use crate::connection_states::ConnectionStateTag;
use crate::error_type::ErrorType;

use std::io::Read;
use std::net::TcpStream;

pub struct PacketReader {
    stream: TcpStream,
}

#[allow(dead_code)]
impl PacketReader {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream: stream }
    }

    pub fn read_packet(
        &mut self,
        state: &ConnectionStateTag,
    ) -> Result<ServerboundPacket, ErrorType> {
        match state {
            ConnectionStateTag::Handshaking => self.read_handshaking_packet(),
            ConnectionStateTag::Status => self.read_status_packet(),
            ConnectionStateTag::Login => self.read_login_packet(),
            ConnectionStateTag::Play => self.read_play_packet(),
            ConnectionStateTag::Exit => unreachable!(),
        }
    }

    fn read_handshaking_packet(&mut self) -> Result<ServerboundPacket, ErrorType> {
        if self.peek_byte()? == 0xfe {
            Ok(ServerboundPacket::LegacyPing(
                LegacyPingServerboundPacket::from_reader(self)?,
            ))
        } else {
            let _len = self.read_varint()?;
            let packet_id = self.read_varint()?;
            match packet_id {
                0x00 => Ok(ServerboundPacket::Handshaking(
                    HandshakingPacket::from_reader(self)?,
                )),
                x => Err(ErrorType::Fatal(format!("Invalid packet {:#04x}", x))),
            }
        }
    }

    fn read_status_packet(&mut self) -> Result<ServerboundPacket, ErrorType> {
        let _len = self.read_varint()?;
        let packet_id = self.read_varint()?;
        match packet_id {
            0x00 => Ok(ServerboundPacket::StatusRequest(
                StatusRequestPacket::from_reader(self)?,
            )),
            0x01 => Ok(ServerboundPacket::Ping(PingPacket::from_reader(self)?)),
            x => Err(ErrorType::Fatal(format!("Invalid packet {:#04x}", x))),
        }
    }

    fn read_login_packet(&mut self) -> Result<ServerboundPacket, ErrorType> {
        let _len = self.read_varint()?;
        let packet_id = self.read_varint()?;
        match packet_id {
            0x00 => Ok(ServerboundPacket::LoginStart(
                LoginStartPacket::from_reader(self)?,
            )),
            x => Err(ErrorType::Fatal(format!("Invalid packet {:#04x}", x))),
        }
    }

    fn read_play_packet(&mut self) -> Result<ServerboundPacket, ErrorType> {
        let _len = self.read_varint()?;
        let packet_id = self.read_varint()?;
        println!("Packet of len {} with id {}", _len, packet_id);
        match packet_id {
            0x03 => Ok(ServerboundPacket::ChatMessage(
                ChatMessagePacket::from_reader(self)?,
            )),
            0x05 => Ok(ServerboundPacket::ClientSettings(
                ClientSettingsPacket::from_reader(self)?,
            )),
            x => Err(ErrorType::Recoverable(format!(
                "Unimplemented packet {:#04x}",
                x
            ))),
        }
    }

    pub fn read_varint(&mut self) -> Result<isize, ErrorType> {
        self.read_var(5)
    }

    pub fn read_varlong(&mut self) -> Result<isize, ErrorType> {
        self.read_var(10)
    }

    fn read_var(&mut self, limit: usize) -> Result<isize, ErrorType> {
        let mut ret: isize = 0;
        let mut num_read = 0;
        loop {
            let read = self.read_unsigned_byte()?;
            ret |= ((read & 0b01111111) as isize) << (7 * num_read);
            num_read += 1;
            if num_read > limit {
                return Err(ErrorType::Recoverable(format!(
                    "Var read {} is out of bounds for {}",
                    num_read, limit
                )));
            } else if (read & 0b10000000) == 0 {
                break;
            }
        }
        Ok(ret)
    }

    fn peek_byte(&mut self) -> Result<u8, ErrorType> {
        let mut buf = [0u8; 1];
        self.stream
            .peek(&mut buf)
            .map_err(|e| ErrorType::Fatal(format!("Peek error: {:?}", e)))?;
        Ok(buf[0])
    }

    pub fn read_unsigned_byte(&mut self) -> Result<u8, ErrorType> {
        let mut buf = [0u8; 1];
        self.stream
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        Ok(buf[0])
    }

    pub fn read_unsigned_short(&mut self) -> Result<u16, ErrorType> {
        let mut buf = [0u8; 2];
        self.stream
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_signed_short(&mut self) -> Result<i16, ErrorType> {
        let mut buf = [0u8; 2];
        self.stream
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_unsigned_int(&mut self) -> Result<u32, ErrorType> {
        let mut buf = [0u8; 4];
        self.stream
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_signed_int(&mut self) -> Result<i32, ErrorType> {
        let mut buf = [0u8; 4];
        self.stream
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        Ok(i32::from_be_bytes(buf))
    }

    pub fn read_character(&mut self) -> Result<char, ErrorType> {
        let mut buf = vec![];
        let mut char_result: Option<Result<char, _>> = None;
        while char_result.is_none() || char_result.unwrap().is_err() {
            let s = self.read_unsigned_short()?;
            buf.push(s);
            char_result = std::char::decode_utf16(buf.iter().cloned())
                .map(|r| r.map_err(|e| e.unpaired_surrogate()))
                .next();
        }
        char_result
            .unwrap()
            .map_err(|e| ErrorType::Fatal(e.to_string()))
    }

    pub fn read_string_chars(&mut self, length: usize) -> Result<String, ErrorType> {
        (0..length).map(|_| self.read_character()).collect()
    }

    pub fn read_string(&mut self) -> Result<String, ErrorType> {
        let len = self.read_varint()?;
        Ok(String::from_utf8(
            (0..len)
                .map(
                    |_| self.read_unsigned_byte().unwrap_or(0x3f), // '?' As replacement
                )
                .collect::<Vec<u8>>(),
        )
        .map_err(|e| ErrorType::Fatal(e.to_string()))?)
    }

    pub fn read_unsigned_long(&mut self) -> Result<u64, ErrorType> {
        let mut buf = [0u8; 8];
        self.stream
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        Ok(u64::from_be_bytes(buf))
    }

    pub fn read_signed_long(&mut self) -> Result<i64, ErrorType> {
        let mut buf = [0u8; 8];
        self.stream
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        Ok(i64::from_be_bytes(buf))
    }

    pub fn read_bool(&mut self) -> Result<bool, ErrorType> {
        let b = self.read_unsigned_byte()?;
        match b {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(ErrorType::Recoverable(format!(
                "Cannot make boolean from {}",
                x
            ))),
        }
    }
}
