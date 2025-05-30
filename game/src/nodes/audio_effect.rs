#[derive(Clone)]
pub struct AudioEffect {
    effect_type: AudioEffectType,
}

#[derive(Clone)]
pub enum AudioEffectType {
    Filter,
    Distortion,
}
impl AudioEffect {
    pub fn new(effect_type: AudioEffectType) -> Self {
        Self { effect_type }
    }
}
