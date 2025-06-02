use std::ops::{Add, Div, Mul};

use crate::core::GameTime;

/// Defines number of ticks in a quarter note
const PULSES_PER_QUARTER_NOTE: u32 = 480;

#[derive(PartialEq, Eq, Clone)]
pub struct NoteGenerator {
    pub loop_length: MusicTime,
    pub notes: Vec<NoteEvent>,
}

impl NoteGenerator {
    pub fn new(loop_length: MusicTime, notes: Vec<NoteEvent>) -> NoteGenerator {
        NoteGenerator { loop_length, notes }
    }

    pub fn from_note_name(note_name: NoteName) -> NoteGenerator {
        // Create a single note in the third octave with 1/4 length
        let note = Note::new(3, note_name);
        let note_event = NoteEvent::new(note, MusicTime::ZERO, NoteDuration::Quarter.into());

        // Create a note generator with a loop length of one quarter note
        NoteGenerator::new(NoteDuration::Quarter.into(), vec![note_event])
    }

    /// Combine multiple note generators into a single one
    pub fn combine(generators: &[NoteGenerator]) -> NoteGenerator {
        if generators.is_empty() {
            return NoteGenerator::new(MusicTime::ZERO, vec![]);
        }

        let mut combined_notes = Vec::new();
        let mut acc_loop_start = MusicTime::ZERO;
        let mut total_loop_length = MusicTime::ZERO;

        for generator in generators {
            for note in &generator.notes {
                combined_notes.push(note.shifted(acc_loop_start));
            }

            // Update the accumulated start time and total loop length
            acc_loop_start = acc_loop_start + generator.loop_length;
            total_loop_length = total_loop_length + generator.loop_length;
        }

        NoteGenerator::new(total_loop_length, combined_notes)
    }
}

/// Enum for ease of use of music durations
#[derive(Clone, Copy)]
pub enum NoteDuration {
    Whole = 0,
    Half = 1,
    Quarter = 2,
    Eighth = 3,
    // Triplets
    Third = 4,
}

impl From<NoteDuration> for MusicTime {
    fn from(value: NoteDuration) -> Self {
        match value {
            NoteDuration::Whole => MusicTime::new(4 * PULSES_PER_QUARTER_NOTE),
            NoteDuration::Half => MusicTime::new(2 * PULSES_PER_QUARTER_NOTE),
            NoteDuration::Quarter => MusicTime::new(PULSES_PER_QUARTER_NOTE),
            NoteDuration::Eighth => MusicTime::new(PULSES_PER_QUARTER_NOTE / 2),
            NoteDuration::Third => MusicTime::new(4 * PULSES_PER_QUARTER_NOTE / 3),
        }
    }
}

/// Represents time in musical terms, independently of BPM
///
/// Given the BPM, you can convert `MusicTime` to `GameTime`
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MusicTime {
    ticks: u32,
}

impl MusicTime {
    pub fn new(ticks: u32) -> Self {
        MusicTime { ticks }
    }

    pub fn to_seconds(&self, bpm: u32) -> GameTime {
        let tick_duration = 60.0 / (bpm as GameTime * PULSES_PER_QUARTER_NOTE as GameTime);
        self.ticks as GameTime * tick_duration
    }

    pub const ZERO: MusicTime = MusicTime { ticks: 0 };
}

impl Add for MusicTime {
    type Output = MusicTime;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ticks: self.ticks + rhs.ticks,
        }
    }
}

impl Mul<u32> for MusicTime {
    type Output = MusicTime;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            ticks: self.ticks * rhs,
        }
    }
}

impl Div<u32> for MusicTime {
    type Output = MusicTime;

    fn div(self, rhs: u32) -> Self::Output {
        Self {
            ticks: self.ticks / rhs,
        }
    }
}

// ---------------------------------- Note ------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NoteEvent {
    pub note: Note,
    pub start: MusicTime,
    pub duration: MusicTime,
}

impl NoteEvent {
    pub fn new(note: Note, start: MusicTime, duration: MusicTime) -> NoteEvent {
        NoteEvent {
            note,
            start,
            duration,
        }
    }

    pub fn shifted(&self, time: MusicTime) -> Self {
        Self {
            start: self.start + time,
            ..*self
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub octave: i32,
    pub note_name: NoteName,
}

impl Note {
    pub fn new(octave: i32, note_name: NoteName) -> Note {
        Note { octave, note_name }
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
        Note::from_semitones(self.to_semitones() + semitones)
    }

    pub fn to_semitones(&self) -> i32 {
        self.octave * 12 + self.note_name.to_int()
    }

    pub fn from_semitones(semitones: i32) -> Note {
        let note_i = semitones.rem_euclid(12);
        let octave = (semitones - note_i) / 12;
        Note {
            octave,
            note_name: NoteName::from_int(note_i as u32),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    pub fn from_int(i: u32) -> NoteName {
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
