use macroquad::color::Color;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;

use super::texture::Texture;
use super::texture::TextureAsset;
use super::Render;
use super::RenderCtx;

#[derive(Clone, Copy)]
pub enum Shape {
    BLANK,
    //Others
    DISTORTION,
    FASTER,
    HIGHPASS,
    LOGO,
    LOWPASS,
    NOTCH,
    PIANO,
    SINE,
    SLOWER,
    SQUARE,
    NOTE,
    REVERB,
    CHORD,
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
            Shape::DISTORTION => {
                Texture::new(position, size, color, TextureAsset::DISTORTION).render(render_ctx)
            }
            Shape::FASTER => {
                Texture::new(position, size, color, TextureAsset::FASTER).render(render_ctx)
            }
            Shape::HIGHPASS => {
                Texture::new(position, size, color, TextureAsset::HIGHPASS).render(render_ctx)
            }
            Shape::LOGO => {
                Texture::new(position, size, color, TextureAsset::LOGO).render(render_ctx)
            }
            Shape::LOWPASS => {
                Texture::new(position, size, color, TextureAsset::LOWPASS).render(render_ctx)
            }
            Shape::NOTCH => {
                Texture::new(position, size, color, TextureAsset::NOTCH).render(render_ctx)
            }
            Shape::NOTE => {
                Texture::new(position, size, color, TextureAsset::NOTE).render(render_ctx)
            }
            Shape::PIANO => {
                Texture::new(position, size, color, TextureAsset::PIANO).render(render_ctx)
            }
            Shape::SINE => {
                Texture::new(position, size, color, TextureAsset::SINE).render(render_ctx)
            }
            Shape::SLOWER => {
                Texture::new(position, size, color, TextureAsset::SLOWER).render(render_ctx)
            }
            Shape::SQUARE => {
                Texture::new(position, size, color, TextureAsset::SQUARE).render(render_ctx)
            }
            Shape::REVERB => {
                Texture::new(position, size, color, TextureAsset::REVERB).render(render_ctx)
            }
            Shape::CHORD => {
                Texture::new(position, size, color, TextureAsset::CHORD).render(render_ctx)
            }
            Shape::BLANK => Ok(()),
        }
    }
}
