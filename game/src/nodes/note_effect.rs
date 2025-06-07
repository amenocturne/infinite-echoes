use crate::nodes::note_generator::Note;
use crate::nodes::note_generator::NoteEvent;
use crate::nodes::note_generator::NoteGenerator;
use crate::nodes::note_generator::NoteName;
use serde::{Deserialize, Serialize};

use super::note_generator::MusicTime;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NoteEffect {
    pub effect_type: NoteEffectType,
}

impl NoteEffect {
    pub fn new(effect_type: NoteEffectType) -> Self {
        Self { effect_type }
    }

    pub fn apply(&self, generator: NoteGenerator) -> NoteGenerator {
        let (transformed_notes, new_loop_length) = match &self.effect_type {
            NoteEffectType::Chord => {
                let notes = generator
                    .notes
                    .into_iter()
                    .flat_map(|event| {
                        let root = event.note;
                        let third = root.shift(4);
                        let fifth = root.shift(7);

                        vec![
                            event.clone(),
                            NoteEvent::new(third, event.start, event.duration),
                            NoteEvent::new(fifth, event.start, event.duration),
                        ]
                    })
                    .collect();
                (notes, generator.loop_length)
            }
            NoteEffectType::Scale(scale) => {
                let notes = generator
                    .notes
                    .into_iter()
                    .flat_map(|event| {
                        let root = event.note;

                        let chord_notes = scale.create_chord_for_note(&root);

                        chord_notes
                            .into_iter()
                            .map(|note| NoteEvent::new(note, event.start, event.duration))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                (notes, generator.loop_length)
            }
            NoteEffectType::ScaleChord(scale) => {
                let notes = generator
                    .notes
                    .into_iter()
                    .flat_map(|event| {
                        let nearest_scale_note = scale.find_nearest_scale_note(&event.note);

                        let chord_notes = scale.create_diatonic_chord(&nearest_scale_note);

                        chord_notes
                            .into_iter()
                            .map(|note| NoteEvent::new(note, event.start, event.duration))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                (notes, generator.loop_length)
            }
            NoteEffectType::ChangeLen(amount) => {
                let len = amount.apply(generator.loop_length);
                let notes = generator
                    .notes
                    .into_iter()
                    .map(|mut event| {
                        event.duration = amount.apply(event.duration);
                        event.start = amount.apply(event.start);
                        event
                    })
                    .collect();
                (notes, len)
            }
        };

        NoteGenerator::new(new_loop_length, transformed_notes)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoteEffectType {
    Chord,
    Scale(Scale),
    ScaleChord(Scale),
    ChangeLen(ChangeLenType),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Scale {
    pub root: NoteName,
    pub scale_type: ScaleType,
}

impl Scale {
    pub fn new(root: NoteName, scale_type: ScaleType) -> Self {
        Scale { root, scale_type }
    }

    pub fn create_chord_for_note(&self, note: &Note) -> Vec<Note> {
        let scale_degrees = self.scale_type.scale_degrees();

        let root_semitones = self.root.to_int();
        let note_semitones = note.note_name.to_int();

        let position_in_scale = scale_degrees
            .iter()
            .position(|&degree| (root_semitones + degree) % 12 == note_semitones % 12);

        if let Some(pos) = position_in_scale {
            let chord_positions = [0, 2, 4];

            let mut chord_notes = Vec::new();
            for offset in &chord_positions {
                let scale_pos = (pos + offset) % scale_degrees.len();
                let degree = scale_degrees[scale_pos];

                let semitones_from_root = (root_semitones + degree) % 12;
                let octave_adjustment = if semitones_from_root < note_semitones % 12 {
                    1
                } else {
                    0
                };

                let new_note = Note::new(
                    note.octave + octave_adjustment,
                    NoteName::from_int(semitones_from_root as u32),
                );

                chord_notes.push(new_note);
            }

            chord_notes
        } else {
            vec![*note]
        }
    }

    pub fn find_nearest_scale_note(&self, note: &Note) -> Note {
        let scale_degrees = self.scale_type.scale_degrees();
        let root_semitones = self.root.to_int();
        let note_semitones = note.note_name.to_int();

        let mut min_distance = 12;
        let mut closest_degree = 0;

        for &degree in &scale_degrees {
            let scale_note_semitones = (root_semitones + degree) % 12;
            let distance = (scale_note_semitones - note_semitones)
                .abs()
                .min((note_semitones + 12 - scale_note_semitones) % 12);

            if distance < min_distance {
                min_distance = distance;
                closest_degree = degree;
            }
        }

        let semitones_from_root = (root_semitones + closest_degree) % 12;
        let octave_adjustment = if semitones_from_root > note_semitones % 12
            && note_semitones % 12 < 6
            && semitones_from_root > 6
        {
            -1
        } else if semitones_from_root < note_semitones % 12
            && note_semitones % 12 > 6
            && semitones_from_root < 6
        {
            1
        } else {
            0
        };

        Note::new(
            note.octave + octave_adjustment,
            NoteName::from_int(semitones_from_root as u32),
        )
    }

    pub fn create_diatonic_chord(&self, note: &Note) -> Vec<Note> {
        let scale_degrees = self.scale_type.scale_degrees();
        let root_semitones = self.root.to_int();
        let note_semitones = note.note_name.to_int();

        let position_in_scale = scale_degrees
            .iter()
            .position(|&degree| (root_semitones + degree) % 12 == note_semitones % 12);

        if let Some(pos) = position_in_scale {
            let mut chord_notes = Vec::new();

            chord_notes.push(*note);

            let third_pos = (pos + 2) % scale_degrees.len();
            let third_degree = scale_degrees[third_pos];
            let third_semitones = (root_semitones + third_degree) % 12;
            let third_octave_adj = if third_semitones < note_semitones % 12 {
                1
            } else {
                0
            };
            chord_notes.push(Note::new(
                note.octave + third_octave_adj,
                NoteName::from_int(third_semitones as u32),
            ));

            let fifth_pos = (pos + 4) % scale_degrees.len();
            let fifth_degree = scale_degrees[fifth_pos];
            let fifth_semitones = (root_semitones + fifth_degree) % 12;
            let fifth_octave_adj = if fifth_semitones < note_semitones % 12 {
                1
            } else {
                0
            };
            chord_notes.push(Note::new(
                note.octave + fifth_octave_adj,
                NoteName::from_int(fifth_semitones as u32),
            ));

            chord_notes
        } else {
            vec![*note]
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ScaleType {
    Major,
    Minor,
}

impl ScaleType {
    fn scale_degrees(&self) -> Vec<i32> {
        match self {
            ScaleType::Major => vec![0, 2, 4, 5, 7, 9, 11],
            ScaleType::Minor => vec![0, 2, 3, 5, 7, 8, 10],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ChangeLenType {
    Double,
    Half,
}

impl ChangeLenType {
    fn apply(&self, time: MusicTime) -> MusicTime {
        match self {
            ChangeLenType::Double => time * 2,
            ChangeLenType::Half => time / 2,
        }
    }
}
