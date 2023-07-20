use super::serverbound::*;

use crate::client_handler::ConnectionStateTag;
use crate::error_type::ErrorType;
use crate::nbt::NBTReader;

use std::net::TcpStream;

pub type PacketReader = NBTReader<TcpStream>;

impl PacketReader {
    pub fn read_packet(
        &mut self,
        state: &ConnectionStateTag,
    ) -> Result<ServerboundPacket, ErrorType> {
        // Length set to MAX because we do not know the length yet, will be reset after the first
        // VarInt is read
        self.curr_packet_length = isize::MAX;
        match state {
            ConnectionStateTag::Handshaking => self.read_handshaking_packet(),
            ConnectionStateTag::Status => self.read_status_packet(),
            ConnectionStateTag::Login => self.read_login_packet(),
            ConnectionStateTag::Play => self.read_play_packet(),
            ConnectionStateTag::Exit => unreachable!(),
        }
    }

    fn read_handshaking_packet(&mut self) -> Result<ServerboundPacket, ErrorType> {
        if self.peek_byte()? == 0xfe {
            // Length set to MAX because we cannot know this in advance
            self.curr_packet_length = isize::MAX;
            self.curr_packet_index = 0;
            Ok(ServerboundPacket::LegacyPing(
                LegacyPingServerboundPacket::from_reader(self)?,
            ))
        } else {
            self.curr_packet_length = self.read_varint()?;
            self.curr_packet_index = 0;
            let packet_id = self.read_varint()?;
            match packet_id {
                0x00 => Ok(ServerboundPacket::Handshaking(
                    HandshakingPacket::from_reader(self)?,
                )),
                x => Err(ErrorType::Fatal(format!("Invalid packet {:#04x}", x))),
            }
        }
    }

    fn read_status_packet(&mut self) -> Result<ServerboundPacket, ErrorType> {
        self.curr_packet_length = self.read_varint()?;
        self.curr_packet_index = 0;
        let packet_id = self.read_varint()?;
        match packet_id {
            0x00 => Ok(ServerboundPacket::StatusRequest(
                StatusRequestPacket::from_reader(self)?,
            )),
            0x01 => Ok(ServerboundPacket::Ping(PingPacket::from_reader(self)?)),
            x => Err(ErrorType::Fatal(format!("Invalid packet {:#04x}", x))),
        }
    }

    fn read_login_packet(&mut self) -> Result<ServerboundPacket, ErrorType> {
        self.curr_packet_length = self.read_varint()?;
        self.curr_packet_index = 0;
        let packet_id = self.read_varint()?;
        match packet_id {
            0x00 => Ok(ServerboundPacket::LoginStart(
                LoginStartPacket::from_reader(self)?,
            )),
            x => Err(ErrorType::Fatal(format!("Invalid packet {:#04x}", x))),
        }
    }

    fn read_play_packet(&mut self) -> Result<ServerboundPacket, ErrorType> {
        self.curr_packet_length = self.read_varint()?;
        self.curr_packet_index = 0;
        let packet_id = self.read_varint()?;
        match packet_id {
            0x00 => Ok(ServerboundPacket::TeleportConfirm(
                TeleportConfirmPacket::from_reader(self)?
            )),
            0x03 => Ok(ServerboundPacket::ChatMessage(
                ChatMessagePacket::from_reader(self)?
            )),
            0x05 => Ok(ServerboundPacket::ClientSettings(
                ClientSettingsPacket::from_reader(self)?
            )),
            0x0b => Ok(ServerboundPacket::PluginMessage(
                PluginMessagePacket::from_reader(self)?
            )),
            0x10 => Ok(ServerboundPacket::KeepAlive(
                KeepAlivePacket::from_reader(self)?
            )),
            0x13 => Ok(ServerboundPacket::PlayerPositionAndRotation(
                PlayerPositionAndRotationPacket::from_reader(self)?
            )),
            0x25 => Ok(ServerboundPacket::HeldItemChange(
                HeldItemChangePacket::from_reader(self)?
            )),
            x => Err(ErrorType::Recoverable(format!(
                "Unimplemented packet {:#04x}",
                x
            ))),
        }
    }

    // Peek should not be necessary for normal NBT parsing, so I put it in PacketReader
    fn peek_byte(&mut self) -> Result<u8, ErrorType> {
        let mut buf = [0u8; 1];
        self.stream
            .peek(&mut buf)
            .map_err(|e| ErrorType::Fatal(format!("Peek error: {:?}", e)))?;
        Ok(buf[0])
    }
}
