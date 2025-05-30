pub mod hover;
pub mod layout;
pub mod rectangle;
pub mod shapes;
pub mod texture;
pub mod widgets;
pub mod rectangle_boundary;
pub mod draggable_card_buffer;
pub mod drag_manager;

use std::collections::HashMap;

use crate::engine::errors::GameError;
use crate::engine::errors::GameResult;
use macroquad::math::Vec2;
use macroquad::texture::load_texture;
use macroquad::texture::Texture2D;
use texture::TextureAsset;
use web_sys::AudioContext;

pub type Assets = HashMap<TextureAsset, Texture2D>;

pub struct RenderCtx {
    assets: Assets,
    screen_size: Vec2,
}

impl RenderCtx {
    pub async fn new(screen_size: Vec2) -> GameResult<Self> {
        let assets = Self::load_assets().await?;
        Ok(RenderCtx {
            assets,
            screen_size,
        })
    }

    async fn load_assets() -> GameResult<Assets> {
        let files = [
            (TextureAsset::Piano, "resources/piano.png"),
            (TextureAsset::SineWave, "resources/sine.png"),
            (TextureAsset::SquareWave, "resources/square.png"),
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
