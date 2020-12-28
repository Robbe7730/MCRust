mod named_nbt_tag;
mod nbt_tag;

pub use named_nbt_tag::*;
pub use nbt_tag::*;

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_into() {
        assert_eq!(NBTTag::Byte(123), 123u8.into());
        assert_eq!(NBTTag::Short(124), 124i16.into());
        assert_eq!(NBTTag::Int(125), 125i32.into());
        assert_eq!(NBTTag::Long(126), 126i64.into());
        assert_eq!(NBTTag::Float(127.0), 127f32.into());
        assert_eq!(NBTTag::Double(128.0), 128f64.into());
        assert_eq!(NBTTag::String("ABC".to_string()), "ABC".into());
        assert_eq!(NBTTag::String("ABD".to_string()), "ABD".to_string().into());
        assert_eq!(
            NBTTag::List(vec![NBTTag::Short(77), NBTTag::Short(30)]),
            vec![77i16, 30i16].into()
        );
        let mut data = HashMap::new();
        data.insert("name", "Test".into());
        data.insert("fame", 4.20f64.into());
        assert_eq!(
            NBTTag::Compound(vec![
                NamedNBTTag::new("name", NBTTag::String("Test".to_string())),
                NamedNBTTag::new("fame", NBTTag::Double(4.20)),
            ]),
            data.into()
        );
    }

    #[test]
    fn test_serialize() {
        // Based on Notch's original test.nbt example
        let hello_world_nbt = NamedNBTTag::new(
            "hello world",
            NBTTag::Compound(vec![NamedNBTTag::new(
                "name",
                NBTTag::String("Bananrama".to_string()),
            )]),
        );

        assert_eq!(
            hello_world_nbt.serialize(),
            [
                0x0a, 0x00, 0x0b, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
                0x08, 0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x09, 0x42, 0x61, 0x6e, 0x61, 0x6e,
                0x72, 0x61, 0x6d, 0x61, 0x00
            ]
        );
    }

    #[test]
    fn test_int_long_array() {
        let int_long_nbt = NamedNBTTag::new(
            "testing different types of array",
            NBTTag::Compound(vec![
                NamedNBTTag::new("intarray", NBTTag::IntArray(vec![i32::MAX, 0, i32::MIN])),
                NamedNBTTag::new("longarray", NBTTag::LongArray(vec![i64::MAX, 0, i64::MIN])),
            ]),
        );

        assert_eq!(
            int_long_nbt.serialize(),
            [
                0x0a, 0x00, 0x20, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x64, 0x69, 0x66,
                0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x6f,
                0x66, 0x20, 0x61, 0x72, 0x72, 0x61, 0x79, 0x0b, 0x00, 0x08, 0x69, 0x6e, 0x74, 0x61,
                0x72, 0x72, 0x61, 0x79, 0x00, 0x00, 0x00, 0x03, 0x7f, 0xff, 0xff, 0xff, 0x00, 0x00,
                0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x61,
                0x72, 0x72, 0x61, 0x79, 0x00, 0x00, 0x00, 0x03, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn test_bigtest_serialize() {
        // Based on Notch's original bigtest.nbt example
        let bigtest_nbt = NamedNBTTag::new(
            "Level",
            NBTTag::Compound(vec![
                NamedNBTTag::new("longTest", NBTTag::Long(9223372036854775807)),
                NamedNBTTag::new("shortTest", NBTTag::Short(32767)),
                NamedNBTTag::new(
                    "stringTest",
                    NBTTag::String("HELLO WORLD THIS IS A TEST STRING ÅÄÖ!".to_string()),
                ),
                NamedNBTTag::new("floatTest", NBTTag::Float(0.49823147)),
                NamedNBTTag::new("intTest", NBTTag::Int(2147483647)),
                NamedNBTTag::new(
                    "nested compound test",
                    NBTTag::Compound(vec![
                        NamedNBTTag::new(
                            "ham",
                            NBTTag::Compound(vec![
                                NamedNBTTag::new("name", NBTTag::String("Hampus".to_string())),
                                NamedNBTTag::new("value", NBTTag::Float(0.75)),
                            ]),
                        ),
                        NamedNBTTag::new(
                            "egg",
                            NBTTag::Compound(vec![
                                NamedNBTTag::new("name", NBTTag::String("Eggbert".to_string())),
                                NamedNBTTag::new("value", NBTTag::Float(0.5)),
                            ]),
                        ),
                    ]),
                ),
                NamedNBTTag::new(
                    "listTest (long)",
                    NBTTag::List(vec![
                        NBTTag::Long(11),
                        NBTTag::Long(12),
                        NBTTag::Long(13),
                        NBTTag::Long(14),
                        NBTTag::Long(15),
                    ]),
                ),
                NamedNBTTag::new(
                    "listTest (compound)",
                    NBTTag::List(vec![
                        NBTTag::Compound(vec![
                            NamedNBTTag::new("name", NBTTag::String("Compound tag #0".to_string())),
                            NamedNBTTag::new("created-on", NBTTag::Long(1264099775885)),
                        ]),
                        NBTTag::Compound(vec![
                            NamedNBTTag::new("name", NBTTag::String("Compound tag #1".to_string())),
                            NamedNBTTag::new("created-on", NBTTag::Long(1264099775885)),
                        ])
                    ])
                ),
                NamedNBTTag::new("byteTest", NBTTag::Byte(127)),
                NamedNBTTag::new(
                    "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))",
                    NBTTag::ByteArray(
                        (0..1000usize).map(|n| {
                            let n_bigger: usize = n as usize;
                            ((n_bigger*n_bigger*255+n_bigger*7)%100) as u8
                        }).collect()
                    )
                ),
                NamedNBTTag::new(
                    "doubleTest",
                    NBTTag::Double(0.4931287132182315)
                )
            ]),
        );
        // TODO: read this from the actual bigtest.nbt
        assert_eq!(
            bigtest_nbt.serialize(),
            [
                0x0a, 0x00, 0x05, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x04, 0x00, 0x08, 0x6c, 0x6f, 0x6e,
                0x67, 0x54, 0x65, 0x73, 0x74, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02,
                0x00, 0x09, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x54, 0x65, 0x73, 0x74, 0x7f, 0xff, 0x08,
                0x00, 0x0a, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x65, 0x73, 0x74, 0x00, 0x29,
                0x48, 0x45, 0x4c, 0x4c, 0x4f, 0x20, 0x57, 0x4f, 0x52, 0x4c, 0x44, 0x20, 0x54, 0x48,
                0x49, 0x53, 0x20, 0x49, 0x53, 0x20, 0x41, 0x20, 0x54, 0x45, 0x53, 0x54, 0x20, 0x53,
                0x54, 0x52, 0x49, 0x4e, 0x47, 0x20, 0xc3, 0x85, 0xc3, 0x84, 0xc3, 0x96, 0x21, 0x05,
                0x00, 0x09, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x54, 0x65, 0x73, 0x74, 0x3e, 0xff, 0x18,
                0x32, 0x03, 0x00, 0x07, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x73, 0x74, 0x7f, 0xff, 0xff,
                0xff, 0x0a, 0x00, 0x14, 0x6e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x6d,
                0x70, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x74, 0x65, 0x73, 0x74, 0x0a, 0x00, 0x03, 0x68,
                0x61, 0x6d, 0x08, 0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x06, 0x48, 0x61, 0x6d,
                0x70, 0x75, 0x73, 0x05, 0x00, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3f, 0x40, 0x00,
                0x00, 0x00, 0x0a, 0x00, 0x03, 0x65, 0x67, 0x67, 0x08, 0x00, 0x04, 0x6e, 0x61, 0x6d,
                0x65, 0x00, 0x07, 0x45, 0x67, 0x67, 0x62, 0x65, 0x72, 0x74, 0x05, 0x00, 0x05, 0x76,
                0x61, 0x6c, 0x75, 0x65, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x0f, 0x6c,
                0x69, 0x73, 0x74, 0x54, 0x65, 0x73, 0x74, 0x20, 0x28, 0x6c, 0x6f, 0x6e, 0x67, 0x29,
                0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x0f, 0x09, 0x00, 0x13, 0x6c, 0x69, 0x73, 0x74, 0x54, 0x65, 0x73, 0x74,
                0x20, 0x28, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x29, 0x0a, 0x00, 0x00,
                0x00, 0x02, 0x08, 0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x0f, 0x43, 0x6f, 0x6d,
                0x70, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x74, 0x61, 0x67, 0x20, 0x23, 0x30, 0x04, 0x00,
                0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x2d, 0x6f, 0x6e, 0x00, 0x00, 0x01,
                0x26, 0x52, 0x37, 0xd5, 0x8d, 0x00, 0x08, 0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x00,
                0x0f, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x74, 0x61, 0x67, 0x20,
                0x23, 0x31, 0x04, 0x00, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x2d, 0x6f,
                0x6e, 0x00, 0x00, 0x01, 0x26, 0x52, 0x37, 0xd5, 0x8d, 0x00, 0x01, 0x00, 0x08, 0x62,
                0x79, 0x74, 0x65, 0x54, 0x65, 0x73, 0x74, 0x7f, 0x07, 0x00, 0x65, 0x62, 0x79, 0x74,
                0x65, 0x41, 0x72, 0x72, 0x61, 0x79, 0x54, 0x65, 0x73, 0x74, 0x20, 0x28, 0x74, 0x68,
                0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x31, 0x30, 0x30, 0x30, 0x20, 0x76,
                0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x28, 0x6e, 0x2a, 0x6e, 0x2a,
                0x32, 0x35, 0x35, 0x2b, 0x6e, 0x2a, 0x37, 0x29, 0x25, 0x31, 0x30, 0x30, 0x2c, 0x20,
                0x73, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
                0x6e, 0x3d, 0x30, 0x20, 0x28, 0x30, 0x2c, 0x20, 0x36, 0x32, 0x2c, 0x20, 0x33, 0x34,
                0x2c, 0x20, 0x31, 0x36, 0x2c, 0x20, 0x38, 0x2c, 0x20, 0x2e, 0x2e, 0x2e, 0x29, 0x29,
                0x00, 0x00, 0x03, 0xe8, 0x00, 0x3e, 0x22, 0x10, 0x08, 0x0a, 0x16, 0x2c, 0x4c, 0x12,
                0x46, 0x20, 0x04, 0x56, 0x4e, 0x50, 0x5c, 0x0e, 0x2e, 0x58, 0x28, 0x02, 0x4a, 0x38,
                0x30, 0x32, 0x3e, 0x54, 0x10, 0x3a, 0x0a, 0x48, 0x2c, 0x1a, 0x12, 0x14, 0x20, 0x36,
                0x56, 0x1c, 0x50, 0x2a, 0x0e, 0x60, 0x58, 0x5a, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0c,
                0x54, 0x42, 0x3a, 0x3c, 0x48, 0x5e, 0x1a, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1c, 0x1e,
                0x2a, 0x40, 0x60, 0x26, 0x5a, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0c, 0x22, 0x42, 0x08,
                0x3c, 0x16, 0x5e, 0x4c, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4e, 0x1e, 0x5c, 0x40, 0x2e,
                0x26, 0x28, 0x34, 0x4a, 0x06, 0x30, 0x00, 0x3e, 0x22, 0x10, 0x08, 0x0a, 0x16, 0x2c,
                0x4c, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4e, 0x50, 0x5c, 0x0e, 0x2e, 0x58, 0x28, 0x02,
                0x4a, 0x38, 0x30, 0x32, 0x3e, 0x54, 0x10, 0x3a, 0x0a, 0x48, 0x2c, 0x1a, 0x12, 0x14,
                0x20, 0x36, 0x56, 0x1c, 0x50, 0x2a, 0x0e, 0x60, 0x58, 0x5a, 0x02, 0x18, 0x38, 0x62,
                0x32, 0x0c, 0x54, 0x42, 0x3a, 0x3c, 0x48, 0x5e, 0x1a, 0x44, 0x14, 0x52, 0x36, 0x24,
                0x1c, 0x1e, 0x2a, 0x40, 0x60, 0x26, 0x5a, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0c, 0x22,
                0x42, 0x08, 0x3c, 0x16, 0x5e, 0x4c, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4e, 0x1e, 0x5c,
                0x40, 0x2e, 0x26, 0x28, 0x34, 0x4a, 0x06, 0x30, 0x00, 0x3e, 0x22, 0x10, 0x08, 0x0a,
                0x16, 0x2c, 0x4c, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4e, 0x50, 0x5c, 0x0e, 0x2e, 0x58,
                0x28, 0x02, 0x4a, 0x38, 0x30, 0x32, 0x3e, 0x54, 0x10, 0x3a, 0x0a, 0x48, 0x2c, 0x1a,
                0x12, 0x14, 0x20, 0x36, 0x56, 0x1c, 0x50, 0x2a, 0x0e, 0x60, 0x58, 0x5a, 0x02, 0x18,
                0x38, 0x62, 0x32, 0x0c, 0x54, 0x42, 0x3a, 0x3c, 0x48, 0x5e, 0x1a, 0x44, 0x14, 0x52,
                0x36, 0x24, 0x1c, 0x1e, 0x2a, 0x40, 0x60, 0x26, 0x5a, 0x34, 0x18, 0x06, 0x62, 0x00,
                0x0c, 0x22, 0x42, 0x08, 0x3c, 0x16, 0x5e, 0x4c, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4e,
                0x1e, 0x5c, 0x40, 0x2e, 0x26, 0x28, 0x34, 0x4a, 0x06, 0x30, 0x00, 0x3e, 0x22, 0x10,
                0x08, 0x0a, 0x16, 0x2c, 0x4c, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4e, 0x50, 0x5c, 0x0e,
                0x2e, 0x58, 0x28, 0x02, 0x4a, 0x38, 0x30, 0x32, 0x3e, 0x54, 0x10, 0x3a, 0x0a, 0x48,
                0x2c, 0x1a, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1c, 0x50, 0x2a, 0x0e, 0x60, 0x58, 0x5a,
                0x02, 0x18, 0x38, 0x62, 0x32, 0x0c, 0x54, 0x42, 0x3a, 0x3c, 0x48, 0x5e, 0x1a, 0x44,
                0x14, 0x52, 0x36, 0x24, 0x1c, 0x1e, 0x2a, 0x40, 0x60, 0x26, 0x5a, 0x34, 0x18, 0x06,
                0x62, 0x00, 0x0c, 0x22, 0x42, 0x08, 0x3c, 0x16, 0x5e, 0x4c, 0x44, 0x46, 0x52, 0x04,
                0x24, 0x4e, 0x1e, 0x5c, 0x40, 0x2e, 0x26, 0x28, 0x34, 0x4a, 0x06, 0x30, 0x00, 0x3e,
                0x22, 0x10, 0x08, 0x0a, 0x16, 0x2c, 0x4c, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4e, 0x50,
                0x5c, 0x0e, 0x2e, 0x58, 0x28, 0x02, 0x4a, 0x38, 0x30, 0x32, 0x3e, 0x54, 0x10, 0x3a,
                0x0a, 0x48, 0x2c, 0x1a, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1c, 0x50, 0x2a, 0x0e, 0x60,
                0x58, 0x5a, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0c, 0x54, 0x42, 0x3a, 0x3c, 0x48, 0x5e,
                0x1a, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1c, 0x1e, 0x2a, 0x40, 0x60, 0x26, 0x5a, 0x34,
                0x18, 0x06, 0x62, 0x00, 0x0c, 0x22, 0x42, 0x08, 0x3c, 0x16, 0x5e, 0x4c, 0x44, 0x46,
                0x52, 0x04, 0x24, 0x4e, 0x1e, 0x5c, 0x40, 0x2e, 0x26, 0x28, 0x34, 0x4a, 0x06, 0x30,
                0x00, 0x3e, 0x22, 0x10, 0x08, 0x0a, 0x16, 0x2c, 0x4c, 0x12, 0x46, 0x20, 0x04, 0x56,
                0x4e, 0x50, 0x5c, 0x0e, 0x2e, 0x58, 0x28, 0x02, 0x4a, 0x38, 0x30, 0x32, 0x3e, 0x54,
                0x10, 0x3a, 0x0a, 0x48, 0x2c, 0x1a, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1c, 0x50, 0x2a,
                0x0e, 0x60, 0x58, 0x5a, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0c, 0x54, 0x42, 0x3a, 0x3c,
                0x48, 0x5e, 0x1a, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1c, 0x1e, 0x2a, 0x40, 0x60, 0x26,
                0x5a, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0c, 0x22, 0x42, 0x08, 0x3c, 0x16, 0x5e, 0x4c,
                0x44, 0x46, 0x52, 0x04, 0x24, 0x4e, 0x1e, 0x5c, 0x40, 0x2e, 0x26, 0x28, 0x34, 0x4a,
                0x06, 0x30, 0x00, 0x3e, 0x22, 0x10, 0x08, 0x0a, 0x16, 0x2c, 0x4c, 0x12, 0x46, 0x20,
                0x04, 0x56, 0x4e, 0x50, 0x5c, 0x0e, 0x2e, 0x58, 0x28, 0x02, 0x4a, 0x38, 0x30, 0x32,
                0x3e, 0x54, 0x10, 0x3a, 0x0a, 0x48, 0x2c, 0x1a, 0x12, 0x14, 0x20, 0x36, 0x56, 0x1c,
                0x50, 0x2a, 0x0e, 0x60, 0x58, 0x5a, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0c, 0x54, 0x42,
                0x3a, 0x3c, 0x48, 0x5e, 0x1a, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1c, 0x1e, 0x2a, 0x40,
                0x60, 0x26, 0x5a, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0c, 0x22, 0x42, 0x08, 0x3c, 0x16,
                0x5e, 0x4c, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4e, 0x1e, 0x5c, 0x40, 0x2e, 0x26, 0x28,
                0x34, 0x4a, 0x06, 0x30, 0x00, 0x3e, 0x22, 0x10, 0x08, 0x0a, 0x16, 0x2c, 0x4c, 0x12,
                0x46, 0x20, 0x04, 0x56, 0x4e, 0x50, 0x5c, 0x0e, 0x2e, 0x58, 0x28, 0x02, 0x4a, 0x38,
                0x30, 0x32, 0x3e, 0x54, 0x10, 0x3a, 0x0a, 0x48, 0x2c, 0x1a, 0x12, 0x14, 0x20, 0x36,
                0x56, 0x1c, 0x50, 0x2a, 0x0e, 0x60, 0x58, 0x5a, 0x02, 0x18, 0x38, 0x62, 0x32, 0x0c,
                0x54, 0x42, 0x3a, 0x3c, 0x48, 0x5e, 0x1a, 0x44, 0x14, 0x52, 0x36, 0x24, 0x1c, 0x1e,
                0x2a, 0x40, 0x60, 0x26, 0x5a, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0c, 0x22, 0x42, 0x08,
                0x3c, 0x16, 0x5e, 0x4c, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4e, 0x1e, 0x5c, 0x40, 0x2e,
                0x26, 0x28, 0x34, 0x4a, 0x06, 0x30, 0x00, 0x3e, 0x22, 0x10, 0x08, 0x0a, 0x16, 0x2c,
                0x4c, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4e, 0x50, 0x5c, 0x0e, 0x2e, 0x58, 0x28, 0x02,
                0x4a, 0x38, 0x30, 0x32, 0x3e, 0x54, 0x10, 0x3a, 0x0a, 0x48, 0x2c, 0x1a, 0x12, 0x14,
                0x20, 0x36, 0x56, 0x1c, 0x50, 0x2a, 0x0e, 0x60, 0x58, 0x5a, 0x02, 0x18, 0x38, 0x62,
                0x32, 0x0c, 0x54, 0x42, 0x3a, 0x3c, 0x48, 0x5e, 0x1a, 0x44, 0x14, 0x52, 0x36, 0x24,
                0x1c, 0x1e, 0x2a, 0x40, 0x60, 0x26, 0x5a, 0x34, 0x18, 0x06, 0x62, 0x00, 0x0c, 0x22,
                0x42, 0x08, 0x3c, 0x16, 0x5e, 0x4c, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4e, 0x1e, 0x5c,
                0x40, 0x2e, 0x26, 0x28, 0x34, 0x4a, 0x06, 0x30, 0x00, 0x3e, 0x22, 0x10, 0x08, 0x0a,
                0x16, 0x2c, 0x4c, 0x12, 0x46, 0x20, 0x04, 0x56, 0x4e, 0x50, 0x5c, 0x0e, 0x2e, 0x58,
                0x28, 0x02, 0x4a, 0x38, 0x30, 0x32, 0x3e, 0x54, 0x10, 0x3a, 0x0a, 0x48, 0x2c, 0x1a,
                0x12, 0x14, 0x20, 0x36, 0x56, 0x1c, 0x50, 0x2a, 0x0e, 0x60, 0x58, 0x5a, 0x02, 0x18,
                0x38, 0x62, 0x32, 0x0c, 0x54, 0x42, 0x3a, 0x3c, 0x48, 0x5e, 0x1a, 0x44, 0x14, 0x52,
                0x36, 0x24, 0x1c, 0x1e, 0x2a, 0x40, 0x60, 0x26, 0x5a, 0x34, 0x18, 0x06, 0x62, 0x00,
                0x0c, 0x22, 0x42, 0x08, 0x3c, 0x16, 0x5e, 0x4c, 0x44, 0x46, 0x52, 0x04, 0x24, 0x4e,
                0x1e, 0x5c, 0x40, 0x2e, 0x26, 0x28, 0x34, 0x4a, 0x06, 0x30, 0x06, 0x00, 0x0a, 0x64,
                0x6f, 0x75, 0x62, 0x6c, 0x65, 0x54, 0x65, 0x73, 0x74, 0x3f, 0xdf, 0x8f, 0x6b, 0xbb,
                0xff, 0x6a, 0x5e, 0x00,
            ]
        );
    }
}