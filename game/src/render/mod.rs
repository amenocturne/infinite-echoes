pub mod rectangle;
pub mod shapes;
pub mod texture;

use std::collections::HashMap;

use crate::engine::errors::{GameError, GameResult};
use macroquad::texture::{load_texture, Texture2D};
use texture::TextureAsset;
use web_sys::AudioContext;

pub type Assets = HashMap<TextureAsset, Texture2D>;

pub struct RenderCtx {
    assets: Assets,
}

impl RenderCtx {
    pub async fn new() -> GameResult<Self> {
        let assets = Self::load_assets().await?;
        Ok(RenderCtx { assets })
    }

    async fn load_assets() -> GameResult<Assets> {
        let files = [
            (TextureAsset::Piano, "resources/piano.png"),
            (TextureAsset::SineWave, "resources/sine.png"),
        ];

        let mut assets = HashMap::new();
        for (a, f) in files {
            let t = load_texture(f)
                .await
                .map_err(|_e| GameError::msg("failed to load asset"))?;
            assets.insert(a, t);
        }

        Ok(assets)
    }
}

pub trait Render {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()>;
}

pub trait RenderAudio {
    fn render_audio(&self, audio_context: &AudioContext) -> GameResult<()>;
}
