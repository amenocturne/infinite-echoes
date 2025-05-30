#[derive(Clone)]
pub struct NoteEffect {
    effect_type: NoteEffectType,
}

#[derive(Clone)]
pub enum NoteEffectType {
    Chord
}
impl NoteEffect {
    pub fn new(effect_type: NoteEffectType) -> Self {
        Self { effect_type }
    }
}
