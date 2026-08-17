use crate::server::ServerStatus;
use rust_mc_status::{ping_java, StatusExt};

pub async fn check() -> Result<ServerStatus, String> {
    let address = "agoniamc.eu";

    match ping_java(address).await {
        Ok(status) => {
            Ok(ServerStatus {
                name: "AgoniaSMP".to_string(),
                address: status.hostname().to_string(),
                online: true,
                players: status.players_online() as u32,
                max_players: status.players_max() as u32,
                version: status.version().to_string(),
                latency: status.latency_ms(),
            })
        }

        Err(error) => Err(error.to_string()),
    }
}