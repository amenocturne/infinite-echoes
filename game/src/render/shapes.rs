use macroquad::{color::Color, math::Vec2};

use crate::engine::errors::GameResult;

use super::{
    texture::{Texture, TextureAsset},
    Render, RenderCtx,
};

#[derive(Clone, Copy)]
pub enum Shape {
    SineWave,
    Piano,
    Blank,
}

impl Shape {
    pub fn draw(
        &self,
        render_ctx: &RenderCtx,
        position: Vec2,
        size: Vec2,
        color: Color,
    ) -> GameResult<()> {
        match self {
            Shape::SineWave => Texture::new(position, size, color, TextureAsset::SineWave).render(render_ctx),
            Shape::Piano => Texture::new(position, size, color, TextureAsset::Piano).render(render_ctx),
            Shape::Blank => Ok(()),
        }
    }
}
