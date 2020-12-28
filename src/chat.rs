// TODO: this is ugly and WIP

use crate::error_type::ErrorType;

use core::convert::TryInto;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    action: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_event: Option<Event>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_event: Option<Event>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Vec<Chat>>,
}

impl Chat {
    pub fn new(text: String) -> Self {
        Chat {
            text,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            color: None,
            insertion: None,
            click_event: None,
            hover_event: None,
            extra: None,
        }
    }
}

#[derive(Debug)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

impl TryInto<ChatMode> for isize {
    type Error = ErrorType;

    fn try_into(self) -> Result<ChatMode, Self::Error> {
        match self {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            x => Err(ErrorType::Recoverable(format!("Invalid chat mode {}", x))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChatPosition {
    NormalMessage,
    SystemMessage,
    AboveHotbar,
}

impl Into<i8> for ChatPosition {
    fn into(self) -> i8 {
        match self {
            ChatPosition::NormalMessage => 0,
            ChatPosition::SystemMessage => 1,
            ChatPosition::AboveHotbar => 2,
        }
    }
}
