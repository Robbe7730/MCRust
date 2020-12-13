use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

use crate::client_handler::ConnectionState;

use crate::packets::ServerboundPacket;
use crate::packets::Serverbound;
use crate::packets::ClientboundPacket;
use crate::packets::Clientbound;

use crate::packets::legacy_ping_serverbound::LegacyPingServerboundPacket;
use crate::packets::handshaking::HandshakingPacket;
use crate::packets::status_request::StatusRequestPacket;

// TODO: Split packet reader into handler containing writer too
pub struct PacketReader {
    stream: TcpStream,
}

#[allow(dead_code)]
impl PacketReader {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
        }
    }

    pub fn read_packet(&mut self, state: ConnectionState) -> Result<ServerboundPacket, String> {
        match state {
            ConnectionState::Handshaking => self.read_handshaking_packet(),
            ConnectionState::Status => self.read_status_packet(),
            _ => Err(format!("Unimplemented state {:?}", state))
        }
    }

    fn read_handshaking_packet(&mut self) -> Result<ServerboundPacket, String> {
        if self.peek_byte()? == 0xfe {
            Ok(ServerboundPacket::LegacyPing(
                    LegacyPingServerboundPacket::from_reader(self)?
              ))
        } else {
            let _len = self.read_varint()?;
            let packet_id = self.read_varint()?;
            match packet_id {
                0x00 => Ok(ServerboundPacket::Handshaking(
                        HandshakingPacket::from_reader(self)?
                )),
                x => Err(format!("Unimplemented packet {:#x}", x))
            }
        }
    }

    fn read_status_packet(&mut self) -> Result<ServerboundPacket, String> {
        let _len = self.read_varint()?;
        let packet_id = self.read_varint()?;
        match packet_id {
            0x00 => Ok(ServerboundPacket::StatusRequest(
                    StatusRequestPacket::from_reader(self)?
            )),
            x => Err(format!("Unimplemented packet {:#x}", x))
        }
    }
    
    pub fn send_packet(&mut self, packet: ClientboundPacket) -> Result<(), String> {
        match self.stream.write_all(&packet.writer().to_bytes()[..]) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn read_varint(&mut self) -> Result<isize, String> {
        self.read_var(5)
    }

    pub fn read_varlong(&mut self) -> Result<isize, String> {
        self.read_var(10)
    }

    fn read_var(&mut self, limit: usize) -> Result<isize, String> {
        let mut ret: isize = 0;
        let mut num_read = 0;
        loop {
            let read = self.read_unsigned_byte()?;
            ret |= ((read & 0b01111111) as isize) << (7 * num_read);
            num_read += 1;
            if num_read > limit {
                return Err(format!("Var read {} is out of bounds for {}", num_read, limit));
            } else if (read & 0b10000000) == 0 {
                break;
            }
        }
        Ok(ret)
    }

    fn peek_byte(&mut self) -> Result<u8, String> {
        let mut buf = [0u8; 1];
        self.stream.peek(&mut buf).map_err(|e| format!("Peek error: {:?}", e))?;
        Ok(buf[0])
    }

    pub fn read_unsigned_byte(&mut self) -> Result<u8, String> {
        let mut buf = [0u8;1];
        self.stream.read_exact(&mut buf).map_err(|x| format!("Read error {:?}", x))?;
        Ok(buf[0])
    }
    
    pub fn read_signed_byte(&mut self) -> Result<i8, String> {
        Ok(self.read_unsigned_byte()? as i8)
    }

    pub fn read_unsigned_short(&mut self) -> Result<u16, String> {
        Ok(((self.read_unsigned_byte()? as u16) << 8) |
                self.read_unsigned_byte()? as u16)
    }

    pub fn read_signed_short(&mut self) -> Result<i16, String> {
        Ok(self.read_unsigned_short()? as i16)
    }

    pub fn read_unsigned_int(&mut self) -> Result<u32, String> {
        Ok(((self.read_unsigned_byte()? as u32) << 24) |
                ((self.read_unsigned_byte()? as u32) << 16) |
                ((self.read_unsigned_byte()? as u32) << 8) |
                self.read_unsigned_byte()? as u32)
    }

    pub fn read_signed_int(&mut self) -> Result<i32, String> {
        Ok(self.read_unsigned_int()? as i32)
    }

    pub fn read_character(&mut self) -> Result<char, String> {
        let mut buf = vec![];
        let mut char_result: Option<Result<char, _>> = None;
        while char_result.is_none() || char_result.unwrap().is_err() {
            let s = self.read_unsigned_short()?;
            buf.push(s);
            char_result = std::char::decode_utf16(buf.iter().cloned())
                                       .map(|r| r.map_err(|e| e.unpaired_surrogate()))
                                       .next();
        }
        char_result.unwrap().map_err(|e| e.to_string())
    }

    pub fn read_string_chars(&mut self, length: usize) -> Result<String, String> {
        (0..length).map(|_| self.read_character())
                    .collect()
    }
    
    pub fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_varint()?;
        Ok(String::from_utf8(
                (0..len).map(
                        |_| self.read_unsigned_byte()
                                .unwrap_or(0x3f) // '?' As replacement
                ).collect::<Vec<u8>>())
            .map_err(|e| e.to_string())?)
    }
}

