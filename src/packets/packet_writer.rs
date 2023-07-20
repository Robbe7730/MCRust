use crate::error_type::ErrorType;
use crate::nbt::{NamedNBTTag, NBTTag};

use std::io::Write;
use std::net::TcpStream;
use std::convert::TryInto;

use uuid::Uuid;

#[derive(Debug)]
pub struct PacketWriter {
    data: Vec<u8>,
    include_length: bool,
}

impl PacketWriter {
    pub fn new(id: u8) -> Self {
        Self {
            data: Self::to_varint(id.into()),
            include_length: true,
        }
    }

    pub fn new_legacy(id: u8) -> Self {
        Self {
            data: vec![id],
            include_length: false,
        }
    }

    pub fn write(&self, mut stream: TcpStream) -> Result<(), ErrorType> {
        if self.include_length {
            let mut data = Self::to_varint(self.data.len().try_into().expect("Too much data"));
            data.extend(&self.data);
            stream
                .write(&data)
                .map_err(|e| ErrorType::Fatal(e.to_string()))?;
        } else {
            stream
                .write(&self.data)
                .map_err(|e| ErrorType::Fatal(e.to_string()))?;
        }

        Ok(())
    }

    pub fn add_unsigned_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn add_signed_byte(&mut self, byte: i8) {
        self.data.push(byte as u8);
    }

    pub fn add_unsigned_short(&mut self, value: u16) {
        self.data.append(&mut value.to_be_bytes().into());
    }

    pub fn add_unsigned_int(&mut self, value: u32) {
        self.data.append(&mut value.to_be_bytes().into());
    }

    pub fn add_signed_double(&mut self, value: f64) {
        self.data.append(&mut value.to_be_bytes().into());
    }

    pub fn add_float(&mut self, value: f32) {
        self.data.append(&mut value.to_be_bytes().into());
    }

    pub fn add_utf16_string(&mut self, value: &String) {
        value
            .encode_utf16()
            .for_each(|x| self.add_unsigned_short(x));
    }

    pub fn add_raw_string(&mut self, value: &String) {
        value.bytes().for_each(|x| self.add_unsigned_byte(x));
    }

    pub fn to_varint(value: i32) -> Vec<u8> {
        let mut mut_value = value as u32;
        let mut ret = vec![];
        loop {
            let mut temp: u8 = (mut_value & 0b01111111) as u8;
            mut_value >>= 7;
            if mut_value != 0 {
                temp |= 0b10000000;
            }
            ret.push(temp);
            if mut_value == 0 {
                break;
            }
        }
        ret
    }

    pub fn add_varint(&mut self, value: i32) {
        for val in Self::to_varint(value) {
            self.add_unsigned_byte(val);
        }
    }

    pub fn add_string(&mut self, value: &String) {
        self.add_varint(value.bytes().len().try_into().expect("String too long"));
        self.add_raw_string(value);
    }

    pub fn add_unsigned_long(&mut self, value: u64) {
        self.data.append(&mut value.to_be_bytes().into());
    }

    pub fn add_signed_long(&mut self, value: i64) {
        self.data.append(&mut value.to_be_bytes().into());
    }

    pub fn add_signed_int(&mut self, value: i32) {
        self.data.append(&mut value.to_be_bytes().into());
    }

    pub fn add_uuid(&mut self, value: Uuid) {
        for &byte in value.as_bytes().iter().rev() {
            self.add_unsigned_byte(byte);
        }
    }

    pub fn add_boolean(&mut self, value: bool) {
        self.data.append(&mut vec![if value { 1 } else { 0 }])
    }

    pub fn add_named_nbt(&mut self, value: &NamedNBTTag) {
        self.data.append(&mut value.serialize());
    }

    pub fn add_nbt(&mut self, value: &NBTTag) {
        self.data.append(&mut value.serialize());
    }

    pub fn add_json(&mut self, value: serde_json::Value) {
        self.add_string(&value.to_string());
    }
}
