#[derive(Clone, Copy, PartialEq)]
pub struct Note {
    pub octave: i32,
    pub note_name: NoteName,
    pub position: NotePosition,
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

    pub fn shift(&self, semitones: i32) -> Note {
        Note::from_semitones(self.to_semitones() + semitones, self.position)
    }

    pub fn to_semitones(&self) -> i32 {
        self.octave * 12 + self.note_name.to_int()
    }

    pub fn from_semitones(semitones: i32, position: NotePosition) -> Note {
        let note_i = semitones.rem_euclid(12);
        let octave = (semitones - note_i) / 12;
        Note {
            octave,
            note_name: NoteName::from_position(note_i),
            position,
        }
    }
}

#[allow(dead_code)]
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

impl NoteName {
    pub fn from_position(i: i32) -> NoteName {
        match i % 12 {
            0 => NoteName::C,
            1 => NoteName::CSharp,
            2 => NoteName::D,
            3 => NoteName::DSharp,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::FSharp,
            7 => NoteName::G,
            8 => NoteName::GSharp,
            9 => NoteName::A,
            10 => NoteName::ASharp,
            11 => NoteName::B,
            _ => unreachable!(),
        }
    }
    pub fn to_int(&self) -> i32 {
        match self {
            NoteName::C => 0,
            NoteName::CSharp => 1,
            NoteName::D => 2,
            NoteName::DSharp => 3,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::FSharp => 6,
            NoteName::G => 7,
            NoteName::GSharp => 8,
            NoteName::A => 9,
            NoteName::ASharp => 10,
            NoteName::B => 11,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct NotePosition {
    pub start: f32,
    pub duration: f32,
}

impl NotePosition {
    pub fn new(start: f32, duration: f32) -> NotePosition {
        NotePosition { start, duration }
    }
}
