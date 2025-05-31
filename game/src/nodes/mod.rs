use crate::nodes::note_generator::Note;
use crate::nodes::note_generator::NoteName;
use audio_effect::AudioEffect;
use audio_effect::AudioEffectType;
use macros::note;
use note_effect::NoteEffect;
use note_effect::NoteEffectType;
use note_generator::MusicTime;
use note_generator::{NoteDuration, NoteEvent, NoteGenerator};
use oscillator::{Oscillator, WaveShape};

use crate::render::widgets::card_widget::CardType;

pub mod audio_effect;
pub mod audio_graph;
pub mod note_effect;
pub mod note_generator;
pub mod oscillator;

#[derive(PartialEq, Eq, Clone)]
pub enum AudioNode {
    NoteGenerator(NoteGenerator),
    NoteEffect(NoteEffect),
    Oscillator(Oscillator),
    AudioEffect(AudioEffect),
}

impl AudioNode {
    pub fn as_note_generator(&self) -> Option<&NoteGenerator> {
        match self {
            AudioNode::NoteGenerator(note_generator) => Some(note_generator),
            _ => None,
        }
    }

    pub fn from_card(card: &CardType) -> Self {
        match card {
            CardType::NoteGenerator => Self::NoteGenerator(NoteGenerator::new(
                NoteDuration::Whole.into(),
                vec![
                    NoteEvent::new(note!("C3"), MusicTime::ZERO, NoteDuration::Quarter.into()),
                    NoteEvent::new(
                        note!("C3"),
                        NoteDuration::Half.into(),
                        NoteDuration::Quarter.into(),
                    ),
                    // NoteEvent::new(
                    //     note!("D3"),
                    //     NoteDuration::Quarter.into(),
                    //     NoteDuration::Quarter.into(),
                    // ),
                    // NoteEvent::new(
                    //     note!("E3"),
                    //     NoteDuration::Half.into(),
                    //     NoteDuration::Quarter.into(),
                    // ),
                ],
            )),
            CardType::SineOscillator => Self::Oscillator(Oscillator::new(WaveShape::Sine)),
            CardType::SquareOscilaltor => Self::Oscillator(Oscillator::new(WaveShape::Square)),
            CardType::AudioEffect => Self::AudioEffect(AudioEffect::new(AudioEffectType::Filter)),
            CardType::NoteEffect => Self::NoteEffect(NoteEffect::new(NoteEffectType::Chord)),
        }
    }
    pub fn to_card_type(&self) -> CardType {
        match self {
            AudioNode::NoteGenerator(_) => CardType::NoteGenerator,
            AudioNode::Oscillator(oscillator) => match oscillator.wave_shape {
                WaveShape::Sine => CardType::SineOscillator,
                WaveShape::Square => CardType::SquareOscilaltor,
            },
            AudioNode::AudioEffect(_) => CardType::AudioEffect,
            AudioNode::NoteEffect(_) => CardType::NoteEffect,
        }
    }

    pub fn as_type(&self) -> AudioNodeType {
        match self {
            AudioNode::NoteGenerator(_) => AudioNodeType::NoteGenerator,
            AudioNode::NoteEffect(_) => AudioNodeType::NoteEffect,
            AudioNode::Oscillator(_) => AudioNodeType::Oscillator,
            AudioNode::AudioEffect(_) => AudioNodeType::AudioEffect,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum AudioNodeType {
    NoteGenerator,
    NoteEffect,
    Oscillator,
    AudioEffect,
}

impl AudioNodeType {
    // Used in actual validation
    pub fn can_put_between_strict(
        &self,
        before: &Option<AudioNodeType>,
        after: &Option<AudioNodeType>,
    ) -> bool {
        match (before, after) {
            (None, None) => true,
            (None, Some(after)) => self.allowed_after(after),
            (Some(before), None) => self.allowed_before(before),
            (Some(before), Some(after)) => self.allowed_before(before) && self.allowed_after(after),
        }
    }

    // Used while building the graph, may lead to invalid graphs
    pub fn can_put_between_loose(
        &self,
        before: &Option<AudioNodeType>,
        after: &Option<AudioNodeType>,
    ) -> bool {
        match (before, after) {
            (None, None) => true,
            (None, Some(after)) => self.allowed_after_loose(after),
            (Some(before), None) => self.allowed_before_loose(before),
            (Some(before), Some(after)) => {
                self.allowed_before_loose(before) && self.allowed_after_loose(after)
            }
        }
    }

    fn allowed_before(&self, t: &AudioNodeType) -> bool {
        match self {
            AudioNodeType::NoteGenerator => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::NoteEffect => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::Oscillator => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::AudioEffect => {
                vec![AudioNodeType::Oscillator, AudioNodeType::AudioEffect]
            }
        }
        .iter()
        .find(|&c| c == t)
        .is_some()
    }

    fn allowed_before_loose(&self, t: &AudioNodeType) -> bool {
        match self {
            AudioNodeType::NoteGenerator => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::NoteEffect => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::Oscillator => {
                vec![AudioNodeType::NoteGenerator, AudioNodeType::NoteEffect]
            }
            AudioNodeType::AudioEffect => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                    AudioNodeType::AudioEffect,
                ]
            }
        }
        .iter()
        .find(|&c| c == t)
        .is_some()
    }

    fn allowed_after(&self, t: &AudioNodeType) -> bool {
        match self {
            AudioNodeType::NoteGenerator => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                ]
            }
            AudioNodeType::NoteEffect => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                ]
            }
            AudioNodeType::Oscillator => {
                vec![AudioNodeType::AudioEffect]
            }
            AudioNodeType::AudioEffect => {
                vec![AudioNodeType::AudioEffect]
            }
        }
        .iter()
        .find(|&c| c == t)
        .is_some()
    }

    // Can lead to invalid states, used only when building audio graph, not validating
    fn allowed_after_loose(&self, t: &AudioNodeType) -> bool {
        match self {
            AudioNodeType::NoteGenerator => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                    AudioNodeType::AudioEffect,
                ]
            }
            AudioNodeType::NoteEffect => {
                vec![
                    AudioNodeType::NoteGenerator,
                    AudioNodeType::NoteEffect,
                    AudioNodeType::Oscillator,
                    AudioNodeType::AudioEffect,
                ]
            }
            AudioNodeType::Oscillator => {
                vec![AudioNodeType::AudioEffect]
            }
            AudioNodeType::AudioEffect => {
                vec![AudioNodeType::AudioEffect]
            }
        }
        .iter()
        .find(|&c| c == t)
        .is_some()
    }
}
