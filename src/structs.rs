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
    extra: Option<Vec<Chat>>
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
