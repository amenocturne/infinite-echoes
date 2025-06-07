use macroquad::color::Color;
use macroquad::math::{vec2, Vec2};

use crate::engine::errors::GameError;
use crate::engine::errors::GameResult;

use super::texture::Texture;
use super::texture::TextureAsset;
use super::Render;
use super::RenderCtx;

#[derive(Clone, Copy)]
pub enum Shape {
    BLANK,
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
    fn to_texture_asset(&self) -> Option<TextureAsset> {
        match self {
            Shape::DISTORTION => Some(TextureAsset::DISTORTION),
            Shape::FASTER => Some(TextureAsset::FASTER),
            Shape::HIGHPASS => Some(TextureAsset::HIGHPASS),
            Shape::LOGO => Some(TextureAsset::LOGO),
            Shape::LOWPASS => Some(TextureAsset::LOWPASS),
            Shape::NOTCH => Some(TextureAsset::NOTCH),
            Shape::NOTE => Some(TextureAsset::NOTE),
            Shape::PIANO => Some(TextureAsset::PIANO),
            Shape::SINE => Some(TextureAsset::SINE),
            Shape::SLOWER => Some(TextureAsset::SLOWER),
            Shape::SQUARE => Some(TextureAsset::SQUARE),
            Shape::REVERB => Some(TextureAsset::REVERB),
            Shape::CHORD => Some(TextureAsset::CHORD),
            Shape::BLANK => None,
        }
    }

    pub fn draw(
        &self,
        render_ctx: &RenderCtx,
        container_top_left: Vec2,
        container_size: Vec2,
        color: Color,
    ) -> GameResult<()> {
        if let Some(texture_asset) = self.to_texture_asset() {
            let texture = render_ctx
                .assets
                .get(&texture_asset)
                .ok_or(GameError::msg("No texture found in render context"))?;

            // Ensure we don't divide by zero if texture or container has no height.
            if texture.height() == 0.0 || container_size.y == 0.0 {
                return Ok(());
            }

            let texture_aspect_ratio = texture.width() / texture.height();
            let container_aspect_ratio = container_size.x / container_size.y;

            let new_size = if container_aspect_ratio > texture_aspect_ratio {
                // Container is wider than the texture's aspect ratio.
                // The height of the container is the limiting dimension.
                vec2(
                    container_size.y * texture_aspect_ratio,
                    container_size.y,
                )
            } else {
                // Container is taller than or has the same aspect ratio as the texture.
                // The width of the container is the limiting dimension.
                vec2(
                    container_size.x,
                    container_size.x / texture_aspect_ratio,
                )
            };

            // Center the new texture size within the container.
            let new_top_left = container_top_left + (container_size - new_size) / 2.0;

            Texture::new(new_top_left, new_size, color, texture_asset).render(render_ctx)
        } else {
            Ok(())
        }
    }
}
