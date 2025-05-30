use std::cell::RefCell;
use std::cmp::min;

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
    pub fn nodes(&self) -> &Vec<RefCell<AudioNode>> {
        &self.nodes
    }

    pub fn as_card_types(&self) -> Vec<CardType> {
        self.nodes
            .iter()
            .map(|n| n.borrow().to_card_type())
            .collect()
    }

    pub fn add(&mut self, node: AudioNode, position: usize) {
        let position = min(position, self.nodes().len());
        self.nodes.insert(position, RefCell::new(node));
    }

    pub fn remove(&mut self, position: usize) {
        let position = min(position, self.nodes.len() - 1);
        if self.nodes.len() != 0 {
            self.nodes.remove(position);
        }
    }

    pub fn replace(&mut self, node: AudioNode, position: usize) {
        let position = min(position, self.nodes.len() - 1);
        self.nodes[position] = RefCell::new(node);
    }

    // pub fn is_valid(&self) -> bool {
    //     let mut checking_stage = CheckingStage::NoteGenerators;
    //     for node in self.nodes() {
    //         match (&checking_stage, node.borrow().as_type()) {
    //             (CheckingStage::NoteGenerators, AudioNodeType::NoteGenerator) => (),
    //             (CheckingStage::NoteGenerators, AudioNodeType::Oscillator) => {
    //                 checking_stage = CheckingStage::AudioEffects;
    //             }
    //             (CheckingStage::AudioEffects, AudioNodeType::AudioEffect) => (),
    //             _ => {
    //                 return false;
    //             }
    //         }
    //     }
    //     true
    // }

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
