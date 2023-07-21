use crate::packets::packet_reader::PacketReader;
use crate::error_type::ErrorType;

use super::Serverbound;

#[derive(Debug)]
pub enum RecipeBook {
    CraftingTable,
    Furnace,
    BlastFurnace,
    Smoker
}

impl RecipeBook {
    fn from_byte(value: u8) -> Result<Self, ErrorType> {
        match value {
            0 => Ok(RecipeBook::CraftingTable),
            1 => Ok(RecipeBook::Furnace),
            2 => Ok(RecipeBook::BlastFurnace),
            3 => Ok(RecipeBook::Smoker),
            x => Err(ErrorType::Recoverable(format!("Invalid recipe book id {}", x)))
        }
    }
}

#[derive(Debug)]
pub struct SetRecipeBookStatePacket {
    pub book_id: RecipeBook,
    pub book_open: bool,
    pub filter_active: bool,
}

impl Serverbound for SetRecipeBookStatePacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {
            book_id: RecipeBook::from_byte(reader.read_unsigned_byte()?)?,
            book_open: reader.read_bool()?,
            filter_active: reader.read_bool()?,
        })
    }
}
