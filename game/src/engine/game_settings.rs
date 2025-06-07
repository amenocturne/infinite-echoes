pub struct GameSettings {
    pub volume: f32,
    pub vault_address: Option<String>,
    pub registry_address: Option<String>,
    pub wallet_address: Option<String>,
    pub is_connected: bool
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            vault_address: None,
            registry_address: None,
            wallet_address: None,
            is_connected: false
        }
    }
}
