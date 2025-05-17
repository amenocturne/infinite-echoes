use macroquad::color::RED;
use macroquad::math::vec2;

use crate::engine::audio::AudioEngine;
use crate::engine::errors::GameResult;
use crate::nodes::audio_graph::AudioGraph;
use crate::render::rectangle::Rectangle;
use crate::render::Render;

pub struct GameState {
    pub audio_engine: AudioEngine,
    pub rectangle: Rectangle,
    pub is_rectangle_visible: bool,
    pub audio_graph: AudioGraph
}

impl GameState {
    pub fn new(audio_engine: AudioEngine, audio_graph: AudioGraph) -> GameState {
        let rectangle = Rectangle::new(vec2(0.0, 0.0), vec2(50.0, 50.0), RED); // TODO: remove
        GameState {
            audio_engine,
            rectangle,
            is_rectangle_visible: false,
            audio_graph
        }
    }
}

impl Render for GameState {
    fn render(&self) -> GameResult<()> {
        if self.is_rectangle_visible {
            self.rectangle.render()?;
        }
        Ok(())
    }
}
//
// impl RenderAudio for GameState {
//     fn render_audio(&self, audio_context: &AudioContext) -> GameResult<()> {
//         self.oscillator.render_audio(audio_context)
//     }
// }

#[derive(PartialEq, Eq, Debug)]
pub enum GameEvent {
    DisplayRectangle,
    InterpretGraph,
    // OscillatorStart,
                      // OscillatorStop,
                      // OscillatorDrag { mouse_pos: Vec2 },
                      // OscillatorSetFrequency { frequency: f32 },
                      // DraggingStart,
                      // DraggingStop,
}
