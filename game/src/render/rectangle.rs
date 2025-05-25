use crate::engine::errors::GameResult;
use crate::render::Render;
use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;

use super::RenderCtx;

pub struct Rectangle {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl Rectangle {
    pub fn new(position: Vec2, size: Vec2, color: Color) -> Rectangle {
        Rectangle {
            position,
            size,
            color,
        }
    }
}

impl Render for Rectangle {
    fn render(&self, _render_ctx: &RenderCtx) -> GameResult<()> {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
        Ok(())
    }
}
