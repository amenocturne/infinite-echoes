use std::cell::RefCell;

use crate::render::widgets::card_widget::CardType;

use super::audio_effect::AudioEffect;
use super::note_generator::NoteGenerator;
use super::oscillator::Oscillator;
use super::{AudioNode, AudioNodeType};

pub struct AudioGraph {
    nodes: Vec<RefCell<AudioNode>>,
}

impl AudioGraph {
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
        if Self::is_valid(cards) {
            None
        } else {
            None
        }
    }

    pub fn nodes(&self) -> &Vec<RefCell<AudioNode>> {
        &self.nodes
    }

    pub fn as_card_types(&self) -> Vec<CardType> {
        self.nodes
            .iter()
            .map(|n| n.borrow().to_card_type())
            .collect()
    }

    fn is_valid(cards: Vec<CardType>) -> bool {
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
        valid && has_oscillator && has_note_generator // TODO: make error msg for the user on what is not ok
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

    // TODO: add function to validate state of the graph when adding/removing nodes
}

enum CheckingStage {
    NoteGenerators,
    AudioEffects,
}
