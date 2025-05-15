pub mod rectangle;

use wasm_bindgen::JsValue;
use web_sys::AudioContext;

pub trait Render {
    fn render(&self) -> ();
}

pub trait RenderAudio {
    fn render_audio(&self, audio_context: &AudioContext) -> Result<(), JsValue>;
}
