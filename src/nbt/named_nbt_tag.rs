use super::NBTTag;

#[derive(Debug, Clone, PartialEq)]
pub struct NamedNBTTag {
    tag: NBTTag,
    name: String,
}

impl NamedNBTTag {
    pub fn new<T: Into<NBTTag>>(name: &str, tag: T) -> Self {
        Self {
            name: name.to_string(),
            tag: tag.into(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut ret = vec![self.tag.type_id()];
        ret.append(&mut NBTTag::String(self.name.clone()).serialize());
        ret.append(&mut self.tag.serialize());
        ret
    }
}
