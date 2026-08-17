#[derive(Debug)]
pub struct ServerStatus {
    pub name: String,
    pub address: String,
    pub online: bool,
    pub players: u32,
    pub max_players: u32,
    pub version: String,
    pub latency: f64,
}