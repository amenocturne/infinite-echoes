pub struct GameSettings {
    pub volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self { volume: 1.0 }
    }
}
