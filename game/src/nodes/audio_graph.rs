use super::note_generator::NoteGenerator;
use super::oscillator::Oscillator;

pub struct AudioGraph {
    pub note_generator: NoteGenerator,
    pub oscillator: Oscillator,
}

impl AudioGraph {
    pub fn new(note_generator: NoteGenerator, oscillator: Oscillator) -> AudioGraph {
        AudioGraph {
            note_generator,
            oscillator,
        }
    }
}
