use super::NamedNBTTag;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NBTTag {
    End,
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NBTTag>),
    Compound(Vec<NamedNBTTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NBTTag {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            NBTTag::End => vec![0x00],
            NBTTag::Byte(x) => x.to_be_bytes().into(),
            NBTTag::Short(x) => x.to_be_bytes().into(),
            NBTTag::Int(x) => x.to_be_bytes().into(),
            NBTTag::Long(x) => x.to_be_bytes().into(),
            NBTTag::Float(x) => x.to_be_bytes().into(),
            NBTTag::Double(x) => x.to_be_bytes().into(),
            NBTTag::ByteArray(bytes) => {
                let mut ret = NBTTag::Int(bytes.len() as i32).serialize();
                ret.append(&mut bytes.clone());
                ret
            }
            NBTTag::String(s) => {
                let mut ret = NBTTag::Short(s.len() as i16).serialize();
                ret.append(&mut s.bytes().collect());
                ret
            }
            NBTTag::List(values) => {
                // This is all assuming the list entries have the same type when required
                // The Into<NBTTag> enforces this but manual construction is still possible
                if values.len() == 0 {
                    let mut ret = NBTTag::End.serialize();
                    ret.append(&mut NBTTag::Int(0).serialize());
                    ret
                } else {
                    let mut ret = NBTTag::Byte(values[0].type_id()).serialize();
                    ret.append(&mut NBTTag::Int(values.len() as i32).serialize());
                    ret.append(
                        &mut values
                            .iter()
                            .map(|value| value.serialize())
                            .flatten()
                            .collect(),
                    );
                    ret
                }
            }
            NBTTag::Compound(values) => {
                let mut ret: Vec<u8> = values
                    .iter()
                    .map(|value| value.serialize())
                    .flatten()
                    .collect();
                ret.append(&mut NBTTag::End.serialize());
                ret
            }
            NBTTag::IntArray(values) => {
                let mut ret = NBTTag::Int(values.len() as i32).serialize();
                ret.append(
                    &mut values
                        .iter()
                        .map(|value| NBTTag::Int(*value).serialize())
                        .flatten()
                        .collect(),
                );
                ret
            }
            NBTTag::LongArray(values) => {
                let mut ret = NBTTag::Int(values.len() as i32).serialize();
                ret.append(
                    &mut values
                        .iter()
                        .map(|value| NBTTag::Long(*value).serialize())
                        .flatten()
                        .collect(),
                );
                ret
            }
        }
    }

    pub fn type_id(&self) -> u8 {
        match self {
            NBTTag::End => 0,
            NBTTag::Byte(_) => 1,
            NBTTag::Short(_) => 2,
            NBTTag::Int(_) => 3,
            NBTTag::Long(_) => 4,
            NBTTag::Float(_) => 5,
            NBTTag::Double(_) => 6,
            NBTTag::ByteArray(_) => 7,
            NBTTag::String(_) => 8,
            NBTTag::List(_) => 9,
            NBTTag::Compound(_) => 10,
            NBTTag::IntArray(_) => 11,
            NBTTag::LongArray(_) => 12,
        }
    }
}

impl Into<NBTTag> for u8 {
    fn into(self) -> NBTTag {
        NBTTag::Byte(self)
    }
}

impl Into<NBTTag> for i16 {
    fn into(self) -> NBTTag {
        NBTTag::Short(self)
    }
}

impl Into<NBTTag> for i32 {
    fn into(self) -> NBTTag {
        NBTTag::Int(self)
    }
}

impl Into<NBTTag> for i64 {
    fn into(self) -> NBTTag {
        NBTTag::Long(self)
    }
}

impl Into<NBTTag> for f32 {
    fn into(self) -> NBTTag {
        NBTTag::Float(self)
    }
}

impl Into<NBTTag> for f64 {
    fn into(self) -> NBTTag {
        NBTTag::Double(self)
    }
}

impl Into<NBTTag> for String {
    fn into(self) -> NBTTag {
        NBTTag::String(self)
    }
}

impl Into<NBTTag> for &str {
    fn into(self) -> NBTTag {
        NBTTag::String(self.to_string())
    }
}

// WARNING: to use ByteArray, IntArray or LongArray, their specific constructors need to be used
impl<T: Into<NBTTag>> Into<NBTTag> for Vec<T> {
    fn into(self) -> NBTTag {
        NBTTag::List(self.into_iter().map(|x| x.into()).collect())
    }
}

impl Into<NBTTag> for HashMap<&str, NBTTag> {
    fn into(self) -> NBTTag {
        NBTTag::Compound(
            self.into_iter()
                .map(|(k, v)| NamedNBTTag::new(k, v))
                .collect(),
        )
    }
}

impl Into<NBTTag> for bool {
    fn into(self) -> NBTTag {
        NBTTag::Byte(self as u8)
    }
}
