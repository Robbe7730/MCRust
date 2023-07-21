#![allow(dead_code)]
use crate::packets::packet_writer::PacketWriter;

#[derive(Copy, Clone, Debug)]
pub enum BrigadierStringArgument {
    SingleWord = 0,
    QuotablePhrase = 1,
    GreedyPhrase = 2
}

#[derive(Debug, Clone)]
pub enum CommandParserType {
    BrigadierBool(),

    // Min, Max
    BrigadierFloat(Option<f32>, Option<f32>),
    BrigadierDouble(Option<f64>, Option<f64>),
    BrigadierInteger(Option<i32>, Option<i32>),
    BrigadierLong(Option<i64>, Option<i64>),

    // Enum argument
    BrigadierString(BrigadierStringArgument),

    // Single match, Only players
    MinecraftEntity(bool, bool),

    // No arguments
    MinecraftGameProfile(),
    MinecraftBlockPos(),
    MinecraftColumnPos(),
    MinecraftVec3(),
    MinecraftVec2(),
    MinecraftBlockState(),
    MinecraftBlockPredicate(),
    MinecraftItemStack(),
    MinecraftItemPredicate(),
    MinecraftColor(),
    MinecraftComponent(),
    MinecraftMessage(),
    MinecraftNBT(),
    MinecraftNBTTag(),
    MinecraftNBTPath(),
    MinecraftObjective(),
    MinecraftObjectiveCriteria(),
    MinecraftOperation(),
    MinecraftParticle(),
    MinecraftAngle(),
    MinecraftRotation(),
    MinecraftScoreboardSlot(),

    // Allow Multiple
    MinecraftScoreHolder(bool),

    // No arguments
    MinecraftSwizzle(),
    MinecraftTeam(),
    MinecraftItemSlot(),
    MinecraftResourceLocation(),
    MinecraftFunction(),
    MinecraftEntityAnchor(),
    MinecraftIntRange(),
    MinecraftFloatRange(),
    MinecraftDimension(),
    MinecraftGamemode(),

    // Minimum duration in ticks
    MinecraftTime(i32),

    // Source registry
    MinecraftResourceOrTag(String),
    MinecraftResourceOfTagKey(String),
    MinecraftResource(String),
    MinecraftResourceKey(String),

    MinecraftTemplateMirror(),
    MinecraftTemplateRotation(),
    MinecraftHeightmap(),
    MinecraftUUID(),
}

impl CommandParserType {
    pub fn to_identifier(&self) -> String {
        (match self {
            CommandParserType::BrigadierBool() => "brigadier:bool",
            CommandParserType::BrigadierFloat(_, _) => "brigadier:float",
            CommandParserType::BrigadierDouble(_, _) => "brigadier:double",
            CommandParserType::BrigadierInteger(_, _) => "brigadier:integer",
            CommandParserType::BrigadierLong(_, _) => "brigadier:long",
            CommandParserType::BrigadierString(_) => "brigadier:string",
            CommandParserType::MinecraftEntity(_, _) => "minecraft:entity",
            CommandParserType::MinecraftGameProfile() => "minecraft:game_profile",
            CommandParserType::MinecraftBlockPos() => "minecraft:block_pos",
            CommandParserType::MinecraftColumnPos() => "minecraft:column_pos",
            CommandParserType::MinecraftVec3() => "minecraft:vec3",
            CommandParserType::MinecraftVec2() => "minecraft:vec2",
            CommandParserType::MinecraftBlockState() => "minecraft:block_state",
            CommandParserType::MinecraftBlockPredicate() => "minecraft:block_predicate",
            CommandParserType::MinecraftItemStack() => "minecraft:item_stack",
            CommandParserType::MinecraftItemPredicate() => "minecraft:item_predicate",
            CommandParserType::MinecraftColor() => "minecraft:color",
            CommandParserType::MinecraftComponent() => "minecraft:component",
            CommandParserType::MinecraftMessage() => "minecraft:message",
            CommandParserType::MinecraftNBT() => "minecraft:nbt",
            CommandParserType::MinecraftNBTTag() => "minecraft:nbt_tag",
            CommandParserType::MinecraftNBTPath() => "minecraft:nbt_path",
            CommandParserType::MinecraftObjective() => "minecraft:objective",
            CommandParserType::MinecraftObjectiveCriteria() => "minecraft:objective_criteria",
            CommandParserType::MinecraftOperation() => "minecraft:operation",
            CommandParserType::MinecraftParticle() => "minecraft:particle",
            CommandParserType::MinecraftAngle() => "minecraft:angle",
            CommandParserType::MinecraftRotation() => "minecraft:rotation",
            CommandParserType::MinecraftScoreboardSlot() => "minecraft:scoreboard_slot",
            CommandParserType::MinecraftScoreHolder(_) => "minecraft:score_holder",
            CommandParserType::MinecraftSwizzle() => "minecraft:swizzle",
            CommandParserType::MinecraftTeam() => "minecraft:team",
            CommandParserType::MinecraftItemSlot() => "minecraft:item_slot",
            CommandParserType::MinecraftResourceLocation() => "minecraft:resource_location",
            CommandParserType::MinecraftFunction() => "minecraft:function",
            CommandParserType::MinecraftEntityAnchor() => "minecraft:entity_anchor",
            CommandParserType::MinecraftIntRange() => "minecraft:int_range",
            CommandParserType::MinecraftFloatRange() => "minecraft:float_range",
            CommandParserType::MinecraftDimension() => "minecraft:dimension",
            CommandParserType::MinecraftGamemode() => "minecraft:gamemode",
            CommandParserType::MinecraftTime(_) => "minecraft:time",
            CommandParserType::MinecraftResourceOrTag(_) => "minecraft:resource_or_tag",
            CommandParserType::MinecraftResourceOfTagKey(_) => "minecraft:resource_or_tag_key",
            CommandParserType::MinecraftResource(_) => "minecraft:resource",
            CommandParserType::MinecraftResourceKey(_) => "minecraft:resource_key",
            CommandParserType::MinecraftTemplateMirror() => "minecraft:template_mirror",
            CommandParserType::MinecraftTemplateRotation() => "minecraft:template_rotation",
            CommandParserType::MinecraftHeightmap() => "minecraft:heightmap",
            CommandParserType::MinecraftUUID() => "minecraft:uuid",
        }).to_string()
    }
}

impl CommandParserType {
    pub fn write(&self, writer: &mut PacketWriter) {
        writer.add_string(&self.to_identifier());
        match self {
            CommandParserType::BrigadierBool() => {},
            CommandParserType::BrigadierFloat(min, max) => {
                let mut flags = 0b00;
                if min.is_some() {
                    flags |= 0b01;
                }
                if max.is_some() {
                    flags |= 0b10;
                }
                writer.add_unsigned_byte(flags);

                if let Some(m) = min {
                    writer.add_float(*m);
                }
                if let Some(m) = max {
                    writer.add_float(*m);
                }
            },
            CommandParserType::BrigadierDouble(min, max) => {
                let mut flags = 0b00;
                if min.is_some() {
                    flags |= 0b01;
                }
                if max.is_some() {
                    flags |= 0b10;
                }
                writer.add_unsigned_byte(flags);

                if let Some(m) = min {
                    writer.add_signed_double(*m);
                }
                if let Some(m) = max {
                    writer.add_signed_double(*m);
                }
            },
            CommandParserType::BrigadierInteger(min, max) => {
                let mut flags = 0b00;
                if min.is_some() {
                    flags |= 0b01;
                }
                if max.is_some() {
                    flags |= 0b10;
                }
                writer.add_unsigned_byte(flags);

                if let Some(m) = min {
                    writer.add_signed_int(*m);
                }
                if let Some(m) = max {
                    writer.add_signed_int(*m);
                }
            },
            CommandParserType::BrigadierLong(min, max) => {
                let mut flags = 0b00;
                if min.is_some() {
                    flags |= 0b01;
                }
                if max.is_some() {
                    flags |= 0b10;
                }
                writer.add_unsigned_byte(flags);

                if let Some(m) = min {
                    writer.add_signed_long(*m);
                }
                if let Some(m) = max {
                    writer.add_signed_long(*m);
                }
            },
            CommandParserType::BrigadierString(value) => {
                writer.add_unsigned_byte(*value as u8);
            },
            CommandParserType::MinecraftEntity(single_match, only_players) => {
                let mut flags = 0b00;
                if *single_match {
                    flags |= 0b01;
                }
                if *only_players {
                    flags |= 0b10;
                }
                writer.add_unsigned_byte(flags);
            },
            CommandParserType::MinecraftGameProfile() => {},
            CommandParserType::MinecraftBlockPos() => {},
            CommandParserType::MinecraftColumnPos() => {},
            CommandParserType::MinecraftVec3() => {},
            CommandParserType::MinecraftVec2() => {},
            CommandParserType::MinecraftBlockState() => {},
            CommandParserType::MinecraftBlockPredicate() => {},
            CommandParserType::MinecraftItemStack() => {},
            CommandParserType::MinecraftItemPredicate() => {},
            CommandParserType::MinecraftColor() => {},
            CommandParserType::MinecraftComponent() => {},
            CommandParserType::MinecraftMessage() => {},
            CommandParserType::MinecraftNBT() => {},
            CommandParserType::MinecraftNBTTag() => {},
            CommandParserType::MinecraftNBTPath() => {},
            CommandParserType::MinecraftObjective() => {},
            CommandParserType::MinecraftObjectiveCriteria() => {},
            CommandParserType::MinecraftOperation() => {},
            CommandParserType::MinecraftParticle() => {},
            CommandParserType::MinecraftAngle() => {},
            CommandParserType::MinecraftRotation() => {},
            CommandParserType::MinecraftScoreboardSlot() => {},
            CommandParserType::MinecraftScoreHolder(allow_multiple) => {
                writer.add_boolean(*allow_multiple);
            },
            CommandParserType::MinecraftSwizzle() => {},
            CommandParserType::MinecraftTeam() => {},
            CommandParserType::MinecraftItemSlot() => {},
            CommandParserType::MinecraftResourceLocation() => {},
            CommandParserType::MinecraftFunction() => {},
            CommandParserType::MinecraftEntityAnchor() => {},
            CommandParserType::MinecraftIntRange() => {},
            CommandParserType::MinecraftFloatRange() => {},
            CommandParserType::MinecraftDimension() => {},
            CommandParserType::MinecraftGamemode() => {},
            CommandParserType::MinecraftTime(min) => {
                writer.add_signed_int(*min);
            },
            CommandParserType::MinecraftResourceOrTag(registry) |
            CommandParserType::MinecraftResourceOfTagKey(registry) |
            CommandParserType::MinecraftResource(registry) | 
            CommandParserType::MinecraftResourceKey(registry) => {
                writer.add_string(registry);
            },
            CommandParserType::MinecraftTemplateMirror() => {},
            CommandParserType::MinecraftTemplateRotation() => {},
            CommandParserType::MinecraftHeightmap() => {},
            CommandParserType::MinecraftUUID() => {},
        }
    }
}
