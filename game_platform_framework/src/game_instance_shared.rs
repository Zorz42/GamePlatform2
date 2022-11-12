pub const RENDERER_ADDRESS: &str = "127.0.0.1:65342";
pub const GAME_ADDRESS: &str = "127.0.0.1:65343";

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum PacketType {
    Clear {
        r: u8,
        g: u8,
        b: u8,
    },
    Refresh,
}