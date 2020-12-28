use super::super::packet_reader::PacketReader;
use super::super::Serverbound;

use crate::error_type::ErrorType;
use crate::chat::ChatMode;

use core::convert::TryInto;

#[derive(Debug)]
pub struct SkinParts {
    cape: bool,
    jacket: bool,
    left_sleeve: bool,
    right_sleeve: bool,
    left_pant_leg: bool,
    right_pant_leg: bool,
    hat: bool
}

impl Into<SkinParts> for u8 {
    fn into(self) -> SkinParts {
        SkinParts {
            cape: (self & 0x01) != 0,
            jacket: (self & 0x02) != 0,
            left_sleeve: (self & 0x04) != 0,
            right_sleeve: (self & 0x08) != 0,
            left_pant_leg: (self & 0x10) != 0,
            right_pant_leg: (self & 0x20) != 0,
            hat: (self & 0x40) != 0
        }
    }
}

#[derive(Debug)]
pub enum Hand {
    Left,
    Right
}

impl TryInto<Hand> for isize {
    type Error = ErrorType;

    fn try_into(self) -> Result<Hand,Self::Error> {
        match self {
            0 => Ok(Hand::Left),
            1 => Ok(Hand::Right),
            x => Err(ErrorType::Recoverable(format!("Invalid hand: {}", x))),
        }
    }
}

#[derive(Debug)]
pub struct ClientSettingsPacket {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: ChatMode,
    pub colors_enabled: bool,
    pub skin_parts_enabled: SkinParts,
    pub main_hand: Hand,
}

impl Serverbound for ClientSettingsPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {
            locale: reader.read_string()?,
            view_distance: reader.read_unsigned_byte()?,
            chat_mode: reader.read_varint()?.try_into()?,
            colors_enabled: reader.read_bool()?,
            skin_parts_enabled: reader.read_unsigned_byte()?.into(),
            main_hand: reader.read_varint()?.try_into()?,
        })
    }
}
