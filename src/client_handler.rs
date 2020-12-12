use std::net::TcpStream;

use crate::packet_reader::PacketReader;

pub struct ClientHandler {
    pub stream: PacketReader,
}

impl ClientHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: PacketReader::new(stream),
        }
    }

    pub fn run(&mut self) {
        println!("RUN!");
        loop {
            match self.stream.read_packet() {
                Ok(_) => println!("YOOO, I READ A PACKET"),
                Err(err_msg) => {
                    println!("Could not read packet :'( {}", err_msg);
                    break
                }
            }
        }
    }
}
