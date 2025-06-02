pub mod drag_manager;
pub mod draggable_card_buffer;
pub mod hover;
pub mod rectangle;
pub mod rectangle_boundary;
pub mod shapes;
pub mod texture;
pub mod widgets;

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
    pub screen_size: Vec2,
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
            (TextureAsset::DISTORTION, "resources/distortion.png"),
            (TextureAsset::FASTER, "resources/faster.png"),
            (TextureAsset::HIGHPASS, "resources/highpass.png"),
            (TextureAsset::LOGO, "resources/logo.png"),
            (TextureAsset::LOWPASS, "resources/lowpass.png"),
            (TextureAsset::NOTCH, "resources/notch.png"),
            (TextureAsset::NOTE, "resources/note.png"),
            (TextureAsset::PIANO, "resources/piano.png"),
            (TextureAsset::SINE, "resources/sine.png"),
            (TextureAsset::SLOWER, "resources/slower.png"),
            (TextureAsset::SQUARE, "resources/square.png"),
            (TextureAsset::REVERB, "resources/reverb.png"),
            (TextureAsset::CHORD, "resources/chord.png"),
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
