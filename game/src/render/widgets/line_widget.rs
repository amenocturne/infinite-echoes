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
        let direction = absolute_start - absolute_end;
        let boundary = direction.normalize() * self.thickness / 2.0;
        draw_line(
            absolute_start.x + boundary.x,
            absolute_start.y + boundary.y,
            absolute_end.x - boundary.x,
            absolute_end.y - boundary.y,
            self.thickness,
            self.color,
        );
        Ok(())
    }
}
