use super::audio_effect::AudioEffect;
use super::note_generator::NoteGenerator;
use super::oscillator::Oscillator;

pub struct AudioGraph {
    pub note_generator: NoteGenerator,
    pub oscillator: Oscillator,
    pub audio_effects: Vec<AudioEffect>,
}

impl AudioGraph {
    pub fn new(
        note_generator: NoteGenerator,
        oscillator: Oscillator,
        audio_effects: Vec<AudioEffect>,
    ) -> AudioGraph {
        AudioGraph {
            note_generator,
            oscillator,
            audio_effects,
        }
    }
}
