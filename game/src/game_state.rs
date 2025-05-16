use macroquad::math::Vec2;
use web_sys::AudioContext;

use crate::{
    engine::audio::AudioEngine,
    errors::GameResult,
    nodes::oscillator::Oscillator,
    render::{Render, RenderAudio},
};

pub struct GameState {
    pub audio_context: AudioContext,
    pub oscillator: Oscillator,
    pub is_dragging: bool,
    pub audio_engine: AudioEngine,
}

impl GameState {
    pub fn new(audio_context: AudioContext, oscillator: Oscillator) -> GameResult<GameState> {
        let audio_engine = AudioEngine::new()?;
        Ok(GameState {
            audio_context,
            oscillator,
            is_dragging: false,
            audio_engine
        })
    }
}

impl Render for GameState {
    fn render(&self) -> GameResult<()> {
        self.oscillator.rectangle.render()
    }
}

impl RenderAudio for GameState {
    fn render_audio(&self, audio_context: &AudioContext) -> GameResult<()> {
        self.oscillator.render_audio(audio_context)
    }
}

pub enum GameEvent {
    OscillatorStart,
    OscillatorStop,
    OscillatorDrag { mouse_pos: Vec2 },
    OscillatorSetFrequency { frequency: f32 },
    DraggingStart,
    DraggingStop,
}
