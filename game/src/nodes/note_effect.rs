#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NoteEffect {
    effect_type: NoteEffectType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NoteEffectType {
    Chord,
}
impl NoteEffect {
    pub fn new(effect_type: NoteEffectType) -> Self {
        Self { effect_type }
    }
}
