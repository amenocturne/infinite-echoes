pub mod rectangle;

use web_sys::AudioContext;
use crate::engine::errors::GameResult;

pub trait Render {
    fn render(&self) -> GameResult<()>;
}

pub trait RenderAudio {
    fn render_audio(&self, audio_context: &AudioContext) -> GameResult<()>;
}
