pub struct NoteGenerator {
    note: Note,
}

pub struct Note {
    octave: i32,
    note_name: NoteName,
    position: NotePosition,
}

impl Note {
    pub fn new(octave: i32, note_name: NoteName, position: NotePosition) -> Note {
        Note {
            octave,
            note_name,
            position,
        }
    }
    pub fn to_frequancy(&self) -> f32 {
        const A3_FREQ: f32 = 440.0;
        const A3_OCTAVE: i32 = 3;

        let semitone_offset = self.note_name as i32 - NoteName::A as i32;
        let octave_diff = self.octave - A3_OCTAVE;
        let semitones_from_a4 = semitone_offset + octave_diff * 12;
        A3_FREQ * 2f32.powf(semitones_from_a4 as f32 / 12.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NoteName {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

pub struct NotePosition {
    start: f32,
    duration: f32,
}

impl NotePosition {
    pub fn new(start: f32, duration: f32) -> NotePosition {
        NotePosition { start, duration }
    }
}
