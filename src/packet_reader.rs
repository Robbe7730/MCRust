use std::net::TcpStream;
use std::io::Read;

use crate::packets::Packet;
use crate::packets::LegacyPingServerbound;

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

    pub fn read_packet(&mut self) -> Result<impl Packet, String> {
        if self.peek_byte()? == 0xfe {
            Ok(LegacyPingServerbound::from_reader(self)?)
        } else {
            // Printing this error also removes the byte from the queue
            Err(format!("Unimplemented packet prefix {:#x}", self.read_unsigned_byte()?))
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
            buf.push(self.read_unsigned_short()?);
            char_result = std::char::decode_utf16(buf.iter().cloned())
                                       .map(|r| r.map_err(|e| e.unpaired_surrogate()))
                                       .next();
        }
        char_result.unwrap().map_err(|e| e.to_string())
    }

    pub fn read_string(&mut self, length: usize) -> Result<String, String> {
        (0..length).map(|_| self.read_character())
                    .collect()
    }
}

