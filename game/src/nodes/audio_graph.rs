use std::ops::Index;

use macroquad::color::BLUE;
use macroquad::math::Vec2;
use macroquad::shapes::draw_line;

use crate::engine::errors::GameResult;
use crate::render::Render;

use super::audio_effect::{self, AudioEffect};
use super::audio_node::AudioNode;
use super::note_generator::NoteGenerator;
use super::oscillator::Oscillator;

pub trait AudioNodeTraits: AudioNode + Render {}

impl<T> AudioNodeTraits for T where T: AudioNode + Render {}

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
    pub fn all_nodes(&self) -> Vec<&dyn AudioNodeTraits> {
        vec![
            vec![
                &self.note_generator as &dyn AudioNodeTraits,
                &self.oscillator as &dyn AudioNodeTraits,
            ],
            self.audio_effects
                .iter()
                .map(|e| e as &dyn AudioNodeTraits)
                .collect(),
        ]
        .concat()
    }

    pub fn is_on_some_node(&self, cursor: &Vec2) -> bool {
        for n in self.all_nodes() {
            if n.is_cursor_on(cursor) {
                return true;
            }
        }
        return false;
    }

    pub fn delete_howered_audio_effect(&mut self, cursor: &Vec2) {
        if let Some(index) = self
            .audio_effects
            .iter()
            .position(|e| e.is_cursor_on(cursor))
        {
            self.audio_effects.remove(index);
        }
    }
}

impl Render for AudioGraph {
    fn render(&self) -> GameResult<()> {
        let ng = &self.note_generator;
        let osc = &self.oscillator;
        let connection = Connection {
            // Create all connections
            start: ng.get_center(),
            end: osc.get_center(),
        };
        connection.render()?;

        let mut last_start = osc.get_center();

        for e in &self.audio_effects {
            let connection = Connection {
                start: last_start,
                end: e.get_center(),
            };
            last_start = e.get_center();
            connection.render()?;
        }

        for n in self.all_nodes() {
            n.render()?;
        }

        Ok(())
    }
}

struct Connection {
    start: Vec2,
    end: Vec2,
}

impl Render for Connection {
    fn render(&self) -> GameResult<()> {
        let thickness = 10.0;
        draw_line(
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y,
            thickness,
            BLUE,
        );
        Ok(())
    }
}
