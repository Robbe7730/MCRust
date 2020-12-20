use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use crate::client_handler::ConnectionState;
use crate::error_type::ErrorType;
use crate::packets::serverbound::*;

pub struct PacketReader {
    stream: Arc<Mutex<TcpStream>>,
}

#[allow(dead_code)]
impl PacketReader {
    pub fn new(stream: Arc<Mutex<TcpStream>>) -> Self {
        Self { stream: stream }
    }

    pub fn read_packet(&mut self, state: ConnectionState) -> Result<ServerboundPacket, ErrorType> {
        match state {
            ConnectionState::Handshaking => self.read_handshaking_packet(),
            ConnectionState::Status => self.read_status_packet(),
            _ => Err(ErrorType::Fatal(format!("Unimplemented state {:?}", state))),
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
                x => Err(ErrorType::Recoverable(format!("Unimplemented packet {}", x))),
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
            x => Err(ErrorType::Recoverable(format!("Unimplemented packet {}", x))),
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
            .lock()
            .map_err(|x| ErrorType::Fatal(format!("Could not lock stream: {}", x.to_string())))?
            .peek(&mut buf)
            .map_err(|e| ErrorType::Fatal(format!("Peek error: {:?}", e)))?;
        Ok(buf[0])
    }

    pub fn read_unsigned_byte(&mut self) -> Result<u8, ErrorType> {
        let mut buf = [0u8; 1];
        self.stream
            .lock()
            .map_err(|x| ErrorType::Fatal(format!("Could not lock stream: {}", x.to_string())))?
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        Ok(buf[0])
    }

    pub fn read_signed_byte(&mut self) -> Result<i8, ErrorType> {
        Ok(self.read_unsigned_byte()? as i8)
    }

    pub fn read_unsigned_short(&mut self) -> Result<u16, ErrorType> {
        Ok(((self.read_unsigned_byte()? as u16) << 8) | self.read_unsigned_byte()? as u16)
    }

    pub fn read_signed_short(&mut self) -> Result<i16, ErrorType> {
        Ok(self.read_unsigned_short()? as i16)
    }

    pub fn read_unsigned_int(&mut self) -> Result<u32, ErrorType> {
        Ok(((self.read_unsigned_byte()? as u32) << 24)
            | ((self.read_unsigned_byte()? as u32) << 16)
            | ((self.read_unsigned_byte()? as u32) << 8)
            | self.read_unsigned_byte()? as u32)
    }

    pub fn read_signed_int(&mut self) -> Result<i32, ErrorType> {
        Ok(self.read_unsigned_int()? as i32)
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
        char_result.unwrap().map_err(|e| ErrorType::Fatal(e.to_string()))
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
        Ok(((self.read_unsigned_byte()? as u64) << 56)
            | ((self.read_unsigned_byte()? as u64) << 48)
            | ((self.read_unsigned_byte()? as u64) << 40)
            | ((self.read_unsigned_byte()? as u64) << 32)
            | ((self.read_unsigned_byte()? as u64) << 24)
            | ((self.read_unsigned_byte()? as u64) << 16)
            | ((self.read_unsigned_byte()? as u64) << 8)
            | self.read_unsigned_byte()? as u64)
    }

pub fn read_signed_long(&mut self) -> Result<i64, ErrorType> {
        Ok(self.read_unsigned_long()? as i64)
    }
}
