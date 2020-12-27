#[derive(Clone)]
pub struct World {
    pub name: String,
    pub seed: [u8; 32],
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}

impl World {
    pub fn dummy() -> Self {
        Self {
            name: "wereld".to_string(),
            seed: [0; 32],
            reduced_debug_info: false,
            enable_respawn_screen: true,
            is_debug: false,
            is_flat: true,
        }
    }
}
