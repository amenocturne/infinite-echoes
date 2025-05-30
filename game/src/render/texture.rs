use macroquad::prelude::*;

use crate::engine::errors::GameError;
use crate::engine::errors::GameResult;

use super::Render;
use super::RenderCtx;

#[derive(Hash, PartialEq, Eq)]
pub enum TextureAsset {
    Piano,
    SineWave,
    SquareWave,
}

pub struct Texture {
    position: Vec2,
    size: Vec2,
    color: Color,
    texture_asset: TextureAsset,
}

impl Texture {
    pub fn new(position: Vec2, size: Vec2, color: Color, texture_asset: TextureAsset) -> Self {
        Texture {
            position,
            size,
            color,
            texture_asset,
        }
    }
}

impl Render for Texture {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        let texture = render_ctx
            .assets
            .get(&self.texture_asset)
            .ok_or(GameError::msg("No texture found in render context"))?;
        let params = DrawTextureParams {
            dest_size: Some(self.size),
            ..Default::default()
        };
        draw_texture_ex(
            texture,
            self.position.x,
            self.position.y,
            self.color,
            params,
        ); // Color's opacity regulates opacity
        Ok(())
    }
}
