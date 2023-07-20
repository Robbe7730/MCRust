use std::io::Read;
use std::convert::{TryInto, TryFrom};

use crate::error_type::ErrorType;

pub struct NBTReader<S: Read> {
    pub stream: S,
    pub curr_packet_index: isize,
    pub curr_packet_length: isize,
}

#[allow(dead_code)]
impl<S: Read> NBTReader<S> {
    pub fn new(stream: S, length: isize) -> Self {
        Self {
            stream,
            curr_packet_index: 0,
            curr_packet_length: length,
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

    pub fn read_unsigned_byte(&mut self) -> Result<u8, ErrorType> {
        let mut buf = [0u8; 1];
        self.read_raw(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_unsigned_short(&mut self) -> Result<u16, ErrorType> {
        let mut buf = [0u8; 2];
        self.read_raw(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_signed_short(&mut self) -> Result<i16, ErrorType> {
        let mut buf = [0u8; 2];
        self.read_raw(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_unsigned_int(&mut self) -> Result<u32, ErrorType> {
        let mut buf = [0u8; 4];
        self.read_raw(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_signed_int(&mut self) -> Result<i32, ErrorType> {
        let mut buf = [0u8; 4];
        self.read_raw(&mut buf)?;
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
        self.read_raw(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    pub fn read_signed_long(&mut self) -> Result<i64, ErrorType> {
        let mut buf = [0u8; 8];
        self.read_raw(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    pub fn read_double(&mut self) -> Result<f64, ErrorType> {
        let mut buf = [0u8; 8];
        self.read_raw(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }

    pub fn read_float(&mut self) -> Result<f32, ErrorType> {
        let mut buf = [0u8; 4];
        self.read_raw(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
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

    pub fn read_until_end(&mut self) -> Result<Vec<u8>, ErrorType> {
        let remaining: usize = (self.curr_packet_length - self.curr_packet_index)
            .try_into()
            .map_err(|_| ErrorType::Fatal("Reading buffer too big".to_string()))?;
        let mut buf = vec![0; remaining];
        self.read_raw(&mut buf)?;
        Ok(buf)
    }

    fn read_raw(&mut self, mut buf: &mut [u8]) -> Result<(), ErrorType>{
        if self.curr_packet_index >= self.curr_packet_length {
            // Could be fatal in PacketReader
            return Err(ErrorType::Recoverable(format!(
                "Reading byte {}, but packet is only {} bytes",
                self.curr_packet_index,
                self.curr_packet_length
            )))
        }
        self.stream
            .read_exact(&mut buf)
            .map_err(|x| ErrorType::Fatal(format!("Read error {:?}", x)))?;
        self.curr_packet_index += isize::try_from(buf.len())
            .map_err(|_| ErrorType::Fatal("Reading buffer too big".to_string()))?;
        Ok(())
    }
}
