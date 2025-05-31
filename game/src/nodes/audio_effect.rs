#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AudioEffect {
    effect_type: AudioEffectType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AudioEffectType {
    Filter,
    Distortion,
}
impl AudioEffect {
    pub fn new(effect_type: AudioEffectType) -> Self {
        Self { effect_type }
    }
}
