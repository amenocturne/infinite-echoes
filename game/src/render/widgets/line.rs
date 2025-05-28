use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_line;

use crate::engine::errors::GameResult;
use crate::render::Render;

pub struct LineWidget {
    start: Vec2,
    end: Vec2,
    thickness: f32,
    color: Color,
}

impl LineWidget {
    pub fn new(start: Vec2, end: Vec2, thickness: f32, color: Color) -> Self {
        Self {
            start,
            end,
            thickness,
            color,
        }
    }
}

impl Render for LineWidget {
    fn render(&self, render_ctx: &crate::render::RenderCtx) -> GameResult<()> {
        let absolute_start = self.start * render_ctx.screen_size;
        let absolute_end = self.end * render_ctx.screen_size;
        draw_line(
            absolute_start.x,
            absolute_start.y,
            absolute_end.x,
            absolute_end.y,
            self.thickness,
            self.color,
        );
        Ok(())
    }
}
