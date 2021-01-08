use uuid::Uuid;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Gamemode {
    pub fn to_byte(&self) -> u8 {
        match &self {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
        }
    }
}

pub fn offline_player_uuid(username: &String) -> Uuid {
    let username_bytes = format!("OfflinePlayer:{}", username)
        .bytes()
        .collect::<Vec<u8>>();
    Uuid::new_v3(&Uuid::NAMESPACE_URL, &username_bytes)
}
