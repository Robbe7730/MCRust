use std::net::TcpListener;

mod minecraft_stream;

use minecraft_stream::MinecraftClient;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565")
                            .expect("Could not start server");

    for stream in listener.incoming() {
        MinecraftClient::new(stream.expect("Invalid stream")).run();
    }
}
