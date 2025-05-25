use macroquad::math::Vec2;

use crate::engine::audio_engine::AudioEngine;
use crate::engine::errors::GameResult;
use crate::nodes::audio_graph::AudioGraph;
use crate::render::{Render, RenderCtx};

pub struct GameState {
    pub audio_engine: AudioEngine,
    pub audio_graph: AudioGraph,
}

impl GameState {
    pub fn new(audio_engine: AudioEngine, audio_graph: AudioGraph) -> GameState {
        GameState {
            audio_engine,
            audio_graph,
        }
    }
}

impl Render for GameState {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        self.audio_graph.render(render_ctx)?;
        Ok(())
    }
}
//
// impl RenderAudio for GameState {
//     fn render_audio(&self, audio_context: &AudioContext) -> GameResult<()> {
//         self.oscillator.render_audio(audio_context)
//     }
// }

pub enum GameEvent {
    InterpretGraph,
    // Audio Node events
    AudioNodeStartDrag(Vec2),
    AudioNodeDrag(Vec2),
    AudioNodeStopDrag,
    AudioNodeAddEffect(Vec2),
    AudioNodeDeleteAudioEffect(Vec2),
    // OscillatorStart,
    // OscillatorStop,
    // OscillatorDrag { mouse_pos: Vec2 },
    // OscillatorSetFrequency { frequency: f32 },
    // DraggingStart,
    // DraggingStop,
}
