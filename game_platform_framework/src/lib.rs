pub mod game_instance_shared;

use laminar;
use bincode;
use crate::game_instance_shared::PacketType;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct GameInstanceManager {
    renderer_address: std::net::SocketAddr,
    socket: laminar::Socket,
}

impl GameInstanceManager {
    pub fn new() -> Self {
        GameInstanceManager{
            renderer_address: game_instance_shared::RENDERER_ADDRESS.parse().unwrap(),
            socket: laminar::Socket::bind(game_instance_shared::GAME_ADDRESS.parse::<String>().unwrap()).unwrap(),
        }
    }

    fn send_packet(&mut self, packet: game_instance_shared::PacketType) {
        self.socket.send(laminar::Packet::unreliable(
            self.renderer_address,
            bincode::serialize(&packet).unwrap(),
        )).unwrap();

        self.socket.manual_poll(std::time::Instant::now());
    }

    pub fn clear_screen(&mut self, color: Color) {
        self.send_packet(PacketType::Clear {
            r: color.r,
            g: color.g,
            b: color.g,
        });
    }

    pub fn refresh_screen(&mut self) {
        self.send_packet(PacketType::Refresh);
    }
}