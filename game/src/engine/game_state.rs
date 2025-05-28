use macroquad::math::Vec2;

use crate::engine::audio_engine::AudioEngine;
use crate::engine::errors::GameResult;
use crate::nodes::audio_graph::AudioGraph;
use crate::render::layout::Layout;
use crate::render::Render;
use crate::render::RenderCtx;

pub struct GameState {
    pub layout: Layout,
    pub audio_engine: AudioEngine,
    pub audio_graph: AudioGraph,
}

impl GameState {
    pub fn new(layout: Layout, audio_engine: AudioEngine, audio_graph: AudioGraph) -> GameState {
        GameState {
            layout,
            audio_engine,
            audio_graph,
        }
    }
}

impl Render for GameState {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        // self.layout.grid.render(render_ctx)?;
        self.audio_graph.render(render_ctx)?;
        Ok(())
    }
}

pub enum GameEvent {
    InterpretGraph,
    ChangeOscillatorType,
    // Audio Node events
    AudioNodeStartDrag(Vec2),
    AudioNodeDrag(Vec2),
    AudioNodeStopDrag,
    AudioNodeAddEffect(Vec2),
    AudioNodeDeleteAudioEffect(Vec2),
}
