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

    pub fn insert_length(&mut self) {
        for val in self.to_varint(self.ret.len()).iter().rev() {
            self.ret.insert(0, *val);
        }
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

    pub fn add_utf16_string(&mut self, value: &String) {
        value.encode_utf16().for_each(|x| self.add_unsigned_short(x));
    }

    pub fn add_raw_string(&mut self, value: &String) {
        value.bytes().for_each(|x| self.add_unsigned_byte(x));
    }

    fn to_varint(&self, value: usize) -> Vec<u8> {
        let mut mut_value = value;
        let mut ret = vec![];
        loop {
            let mut temp: u8 = (mut_value & 0b01111111) as u8;
            mut_value >>= 7;
            if mut_value != 0 {
                temp |= 0b10000000;
            }
            ret.push(temp);
            if mut_value == 0 {
                break;
            }
        } 
        ret

    }

    pub fn add_varint(&mut self, value: usize) {
        for val in self.to_varint(value) {
            self.add_unsigned_byte(val);
        }
    }
    
    pub fn add_string(&mut self, value: &String) {
        self.add_varint(value.bytes().len());
        self.add_raw_string(value);
    }
}
