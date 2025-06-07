use std::cell::RefCell;

use crate::render::widgets::card_widget::CardType;

use super::audio_effect::AudioEffect;
use super::note_effect::NoteEffect;
use super::note_generator::NoteGenerator;
use super::oscillator::Oscillator;
use super::{AudioNode, AudioNodeType};

#[derive(PartialEq, Clone)]
pub struct AudioGraph {
    nodes: Vec<RefCell<AudioNode>>,
}

impl AudioGraph {
    /// Process the audio graph and return a list of processed note generators
    /// Groups note generators into blocks, applies effects to the combined group,
    /// and returns the processed blocks in sequence
    pub fn process_note_generators(&self) -> NoteGenerator {
        let mut blocks: Vec<(Vec<NoteGenerator>, Vec<NoteEffect>)> = Vec::new();
        let mut current_generators: Vec<NoteGenerator> = Vec::new();
        let mut current_effects: Vec<NoteEffect> = Vec::new();
        let mut consuming_effects = false;

        for node_ref in &self.nodes {
            match &*node_ref.borrow() {
                AudioNode::NoteGenerator(ng) => {
                    if !consuming_effects {
                        current_generators.push(ng.clone());
                    } else {
                        consuming_effects = false;
                        blocks.push((current_generators, current_effects));
                        current_generators = Vec::new();
                        current_generators.push(ng.clone());
                        current_effects = Vec::new();
                    }
                }
                AudioNode::NoteEffect(effect) => {
                    consuming_effects = true;
                    current_effects.push(effect.clone());
                }
                AudioNode::Oscillator(_) | AudioNode::AudioEffect(_) => {
                    if !current_generators.is_empty() {
                        blocks.push((current_generators, current_effects));
                        current_generators = Vec::new();
                        current_effects = Vec::new();
                    }
                    consuming_effects = false;
                }
            }
        }

        if !current_generators.is_empty() {
            blocks.push((current_generators, current_effects));
        }

        let mut result: Vec<NoteGenerator> = Vec::new();
        for (generators, effects) in blocks {
            if generators.is_empty() {
                continue;
            }
            let combined_generator = NoteGenerator::combine(&generators);
            let mut processed_generator = combined_generator;
            for effect in &effects {
                processed_generator = effect.apply(processed_generator);
            }
            result.push(processed_generator);
        }

        NoteGenerator::combine(result.as_slice())
    }

    pub fn note_effects(&self) -> Vec<NoteEffect> {
        self.nodes
            .iter()
            .filter_map(|node| {
                let borrowed = node.borrow();
                if let AudioNode::NoteEffect(ref effect) = *borrowed {
                    Some(effect.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn new(
        note_generators: Vec<NoteGenerator>,
        oscillator: Oscillator,
        audio_effects: Vec<AudioEffect>,
    ) -> AudioGraph {
        let ngs = note_generators
            .iter()
            .map(|ng| RefCell::new(AudioNode::NoteGenerator(ng.clone())))
            .collect();

        let osc = vec![RefCell::new(AudioNode::Oscillator(oscillator.clone()))];
        let aes = audio_effects
            .iter()
            .map(|ae| RefCell::new(AudioNode::AudioEffect(ae.clone())))
            .collect();

        let nodes = vec![ngs, osc, aes].concat();

        AudioGraph { nodes }
    }

    pub fn from_cards(cards: Vec<CardType>) -> Option<Self> {
        if Self::is_valid(&cards) {
            let nodes = cards
                .iter()
                .map(|c| RefCell::new(AudioNode::from_card(c)))
                .collect();
            Some(Self { nodes })
        } else {
            None
        }
    }

    pub fn nodes(&self) -> &Vec<RefCell<AudioNode>> {
        &self.nodes
    }

    fn is_valid(cards: &Vec<CardType>) -> bool {
        let mut maybe_before = None;
        let mut maybe_current = None;
        let mut valid = true;
        let mut has_note_generator = false;
        let mut has_oscillator = false;
        for card in cards.iter() {
            match card.as_type() {
                AudioNodeType::NoteGenerator => {
                    has_note_generator = true;
                }
                AudioNodeType::Oscillator => {
                    has_oscillator = true;
                }
                _ => (),
            }
            if maybe_current.is_none() {
                maybe_current = Some(card);
                continue;
            }
            let maybe_after = Some(card.as_type());
            if let Some(checking_node) = maybe_current {
                valid = valid
                    && checking_node
                        .as_type()
                        .can_put_between_strict(&maybe_before, &maybe_after);
                maybe_before = Some(checking_node.as_type());
            }

            maybe_current = Some(card);
        }
        valid && has_oscillator && has_note_generator
    }

    pub fn note_generators(&self) -> Vec<NoteGenerator> {
        self.nodes
            .iter()
            .filter_map(|node| {
                let borrowed = node.borrow();
                if let AudioNode::NoteGenerator(ref ng) = *borrowed {
                    Some(ng.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn oscillator(&self) -> Option<Oscillator> {
        self.nodes
            .iter()
            .filter_map(|node| {
                let borrowed = node.borrow();
                if let AudioNode::Oscillator(ref osc) = *borrowed {
                    Some(osc.clone())
                } else {
                    None
                }
            })
            .next()
    }

    pub fn audio_effects(&self) -> Vec<AudioEffect> {
        self.nodes
            .iter()
            .filter_map(|node| {
                let borrowed = node.borrow();
                if let AudioNode::AudioEffect(ref ae) = *borrowed {
                    Some(ae.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

enum CheckingStage {
    NoteGenerators,
    AudioEffects,
}
