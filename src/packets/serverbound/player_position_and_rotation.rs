use crate::packets::packet_reader::PacketReader;
use crate::error_type::ErrorType;

use super::Serverbound;

#[derive(Debug)]
pub struct PlayerPositionAndRotationPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl Serverbound for PlayerPositionAndRotationPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(PlayerPositionAndRotationPacket {
            x: reader.read_double()?,
            feet_y: reader.read_double()?,
            z: reader.read_double()?,
            yaw: reader.read_float()?,
            pitch: reader.read_float()?,
            on_ground: reader.read_bool()?,
        })
    }
}
