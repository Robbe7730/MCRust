use super::super::Clientbound;
use super::super::packet_writer::PacketWriter;

use rand::random;

pub enum ValueType<T> {
    Absolute(T),
    Relative(T)
}

impl<T> ValueType<T> {
    pub fn get_value(&self) -> &T {
        match self {
            ValueType::Absolute(t) => t,
            ValueType::Relative(t) => t,
        }
    }

    pub fn is_relative(&self) -> bool {
        match self {
            ValueType::Absolute(_) => false,
            ValueType::Relative(_) => true,
        }
    }
}

pub struct PlayerPositionAndLookPacket {
    pub x: ValueType<i64>,
    pub y: ValueType<i64>,
    pub z: ValueType<i64>,
    pub yaw: ValueType<f32>,
    pub pitch: ValueType<f32>,
    pub teleport_id: i32,
}

impl Clientbound for PlayerPositionAndLookPacket {
    fn writer(&self) -> PacketWriter {
        let mut flags: u8 = 0;
        if self.x.is_relative() {
            flags |= 0x01;
        }
        if self.y.is_relative() {
            flags |= 0x02;
        }
        if self.z.is_relative() {
            flags |= 0x04;
        }
        if self.pitch.is_relative() {
            flags |= 0x08;
        }
        if self.yaw.is_relative() {
            flags |= 0x10;
        }

        let mut writer = PacketWriter::new(0x34);
        writer.add_signed_double(*self.x.get_value());
        writer.add_signed_double(*self.y.get_value());
        writer.add_signed_double(*self.z.get_value());
        writer.add_float(*self.yaw.get_value());
        writer.add_float(*self.pitch.get_value());
        writer.add_unsigned_byte(flags);
        writer.add_varint(self.teleport_id);
        writer
    }
}
