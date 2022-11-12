pub mod game_instance_shared;

use laminar;
use serde_derive;
use bincode;

pub fn execute() {
    let renderer_address = game_instance_shared::RENDERER_ADDRESS.parse().unwrap();
    let mut socket = laminar::Socket::bind(game_instance_shared::GAME_ADDRESS.parse::<String>().unwrap()).unwrap();

    socket.send(laminar::Packet::unreliable(
        renderer_address,
        bincode::serialize(&game_instance_shared::PacketType::Clear {
            r: 255,
            g: 255,
            b: 0,
        }).unwrap(),
    )).unwrap();

    socket.send(laminar::Packet::unreliable(
        renderer_address,
        bincode::serialize(&game_instance_shared::PacketType::Refresh).unwrap(),
    )).unwrap();

    socket.manual_poll(std::time::Instant::now());

    std::thread::sleep(std::time::Duration::from_secs(1));

    socket.send(laminar::Packet::unreliable(
        renderer_address,
        bincode::serialize(&game_instance_shared::PacketType::Clear {
            r: 0,
            g: 0,
            b: 0,
        }).unwrap(),
    )).unwrap();

    socket.send(laminar::Packet::unreliable(
        renderer_address,
        bincode::serialize(&game_instance_shared::PacketType::Refresh).unwrap(),
    )).unwrap();

    socket.manual_poll(std::time::Instant::now());

    std::thread::sleep(std::time::Duration::from_secs(1));
}