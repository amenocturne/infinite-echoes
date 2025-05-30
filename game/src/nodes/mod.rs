use audio_effect::AudioEffect;
use note_generator::NoteGenerator;
use oscillator::{Oscillator, WaveShape};

use crate::render::widgets::card_widget::CardType;

pub mod audio_effect;
pub mod audio_graph;
pub mod note_generator;
pub mod oscillator;

#[derive(Clone)]
pub enum AudioNode {
    NoteGenerator(NoteGenerator),
    Oscillator(Oscillator),
    AudioEffect(AudioEffect),
}

impl AudioNode {
    pub fn to_card_type(&self) -> CardType {
        match self {
            AudioNode::NoteGenerator(_) => CardType::NoteGenerator,
            AudioNode::Oscillator(oscillator) => match oscillator.wave_shape {
                WaveShape::Sine => CardType::SineOscillator,
                WaveShape::Square => CardType::SquareOscilaltor,
            },
            AudioNode::AudioEffect(_) => CardType::AudioEffect,
        }
    }

    pub fn as_type(&self) -> AudioNodeType {
        match self {
            AudioNode::NoteGenerator(_) => AudioNodeType::NoteGenerator,
            AudioNode::Oscillator(_) => AudioNodeType::Oscillator,
            AudioNode::AudioEffect(_) => AudioNodeType::AudioEffect,
        }
    }
}

#[derive(Clone)]
pub enum AudioNodeType {
    NoteGenerator,
    Oscillator,
    AudioEffect,
}
