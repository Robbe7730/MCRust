use std::convert::TryInto;

use crate::packets::packet_writer::PacketWriter;

pub use super::command_parser::CommandParserType;

#[derive(Debug, Clone)]
pub enum CommandSuggestionType {
    MinecraftAskServer,
    MinecraftAllRecipes,
    MinecraftAvailableSounds,
    MinecraftAvailableBiomes,
    MinecraftSummonableEntities,
}

impl Into<String> for &CommandSuggestionType {
    fn into(self) -> String {
        match self {
            CommandSuggestionType::MinecraftAskServer => "minecraft:ask_server".to_string(),
            CommandSuggestionType::MinecraftAllRecipes => "minecraft:all_recipes".to_string(),
            CommandSuggestionType::MinecraftAvailableSounds => "minecraft:available_sounds".to_string(),
            CommandSuggestionType::MinecraftAvailableBiomes => "minecraft:available_biomes".to_string(),
            CommandSuggestionType::MinecraftSummonableEntities => "minecraft:summonable_entities".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandNode {
    // Executable, Children, Redirect
    Root(bool, Vec<i32>, Option<i32>),

    // Executable, Children, Redirect, Name
    Literal(bool, Vec<i32>, Option<i32>, String),

    // Executable, Children, Redirect, Name, Parser, Suggestion Type
    Argument(bool, Vec<i32>, Option<i32>, String, CommandParserType, Option<CommandSuggestionType>),
}

impl CommandNode {
    pub fn write(&self, writer: &mut PacketWriter) {
        // Write flags
        let mut flags = match self {
            CommandNode::Root(_, _, _) => 0b00,
            CommandNode::Literal(_, _, _, _) => 0b01,
            CommandNode::Argument(_, _, _, _, _, _) => 0b10,
        };

        match self {
            CommandNode::Root(executable, _, maybe_redirect) |
            CommandNode::Literal(executable, _, maybe_redirect, _) |
            CommandNode::Argument(executable, _, maybe_redirect, _, _, _)=> {
                if *executable {
                    flags |= 0b00000100;
                }
                if maybe_redirect.is_some() {
                    flags |= 0b00001000;
                }
            }
        }

        if let CommandNode::Argument(_, _, _, _, _, Some(_)) = self {
            flags |= 0b00010000;
        }

        writer.add_unsigned_byte(flags);

        // Write children & redirect
        match self {
            CommandNode::Root(_, children, maybe_redirect) |
            CommandNode::Literal(_, children, maybe_redirect, _) |
            CommandNode::Argument(_, children, maybe_redirect, _, _, _)=> {
                writer.add_varint(children.len().try_into().unwrap());

                for child in children {
                    writer.add_varint(*child);
                }

                if let Some(redirect) = maybe_redirect {
                    writer.add_varint(*redirect);
                }
            }
        }

        // Write name
        match self {
            CommandNode::Root(_, _, _) => {},
            CommandNode::Literal(_, _, _, name) |
                CommandNode::Argument(_, _, _, name, _, _) => {
                writer.add_string(name);
            }
        }

        // Write argument-specific parts
        if let CommandNode::Argument(_, _, _, _, parser_id, maybe_suggestions) = self {
            parser_id.write(writer);

            if let Some(suggestions) = maybe_suggestions {
                writer.add_string(&suggestions.into());
            }
        }
    }
}
