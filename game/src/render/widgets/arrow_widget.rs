use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_line;

use crate::engine::errors::GameResult;
use crate::render::Render;

pub struct ArrowWidget {
    start: Vec2,
    end: Vec2,
    thickness: f32,
    color: Color,
    head_length: f32,
    head_width: f32,
}

impl ArrowWidget {
    pub fn new(start: Vec2, end: Vec2, thickness: f32, color: Color) -> Self {
        let length = (end - start).length();
        Self {
            start,
            end,
            thickness,
            color,
            head_length: 0.2 * length,
            head_width: 0.1 * length,
        }
    }

    pub fn with_head_size(mut self, head_length: f32, head_width: f32) -> Self {
        self.head_length = head_length;
        self.head_width = head_width;
        self
    }
}

impl Render for ArrowWidget {
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

        let dx = absolute_end.x - absolute_start.x;
        let dy = absolute_end.y - absolute_start.y;
        let length = (absolute_end - absolute_start).length();

        if length > 0.0 {
            let unit_x = dx / length;
            let unit_y = dy / length;

            let head_length_px =
                self.head_length * render_ctx.screen_size.x.min(render_ctx.screen_size.y);
            let head_width_px =
                self.head_width * render_ctx.screen_size.x.min(render_ctx.screen_size.y);

            let head_x1 = absolute_end.x - head_length_px * unit_x + head_width_px * unit_y;
            let head_y1 = absolute_end.y - head_length_px * unit_y - head_width_px * unit_x;
            let head_x2 = absolute_end.x - head_length_px * unit_x - head_width_px * unit_y;
            let head_y2 = absolute_end.y - head_length_px * unit_y + head_width_px * unit_x;

            draw_line(
                absolute_end.x,
                absolute_end.y,
                head_x1,
                head_y1,
                self.thickness,
                self.color,
            );
            draw_line(
                absolute_end.x,
                absolute_end.y,
                head_x2,
                head_y2,
                self.thickness,
                self.color,
            );
        }

        Ok(())
    }
}
