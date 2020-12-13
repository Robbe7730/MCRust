pub struct PacketWriter {
    ret: Vec<u8>,
}

#[allow(unused)]
impl PacketWriter {
    pub fn new() -> Self {
        Self {
            ret: vec![]
        }
    }

    pub fn to_bytes(&self) -> &Vec<u8> {
        &self.ret
    }

    pub fn add_unsigned_byte(&mut self, byte: u8) {
        self.ret.push(byte);
    }

    pub fn add_signed_byte(&mut self, byte: i8) {
        self.add_unsigned_byte(byte as u8);
    }

    pub fn add_unsigned_short(&mut self, value: u16) {
        self.ret.push((value >> 8) as u8);
        self.ret.push((value & 0xff) as u8);
    }

    pub fn add_signed_short(&mut self, signed_value: i16) {
        self.add_unsigned_short(signed_value as u16);
    }

    pub fn add_string(&mut self, value: &String) {
        value.encode_utf16().for_each(|x| self.add_unsigned_short(x));
    }
    
    pub fn add_string_null_terminated(&mut self, value: &String) {
        self.add_string(value);
        self.add_unsigned_short(0x0000);
    }
}
