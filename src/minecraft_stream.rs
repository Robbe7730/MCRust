use std::net::TcpStream;
use std::io::Read;

struct MinecraftStream {
    stream: TcpStream,
}

impl MinecraftStream {
    pub fn read_varint(&mut self) -> usize {
        let mut buf = [0u8;1];
        let mut ret: usize = 0;
        let mut num_read = 0;
        loop {
            if let Ok(()) = self.stream.read_exact(&mut buf) {
                if (buf[0] & 0b10000000) == 0 {
                    break;
                }
                ret |= ((buf[0] & 0b01111111) as usize) << (7 * num_read);
                num_read += 1;
                if num_read > 5 {
                    break;
                }
            } else {
                break;
            }
        }
        ret
    }
}

pub struct MinecraftClient {
    stream: MinecraftStream,
}

impl MinecraftClient {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: MinecraftStream { stream },
        }
    }

    pub fn run(&mut self) {
        println!("RUN!");
        println!("{}", self.stream.read_varint());
    }
}
