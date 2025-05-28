use macroquad::{
    color::WHITE,
    math::{vec2, Vec2},
    shapes::draw_line,
};
use miniquad::window::screen_size;

use crate::engine::errors::GameResult;

use super::Render;

pub struct Layout {
    pub grid: GridWidget,
}

impl Layout {
    pub fn new(grid: GridWidget) -> Self {
        Layout { grid }
    }
}

/// Position and size are in percent to the whole screen width and height
pub struct GridWidget {
    position: Vec2,
    size: Vec2,
    columns: u32,
    rows: u32,
}

impl GridWidget {
    pub fn new(position: Vec2, size: Vec2, columns: u32, rows: u32) -> Self {
        GridWidget {
            position,
            size,
            columns,
            rows,
        }
    }
    pub fn snapping_points(&self) -> Vec<Vec2> {
        let mut points = vec![];
        let cell_size = self.size / vec2(self.columns as f32, self.rows as f32);
        for col in 0..self.columns {
            for row in 0..self.rows {
                points.push(vec2(
                    cell_size.x / 2.0 + col as f32 * cell_size.x + self.position.x,
                    cell_size.y / 2.0 + row as f32 * cell_size.y + self.position.y,
                ))
            }
        }
        return points;
    }
}

impl Render for GridWidget {
    fn render(&self, render_ctx: &super::RenderCtx) -> GameResult<()> {
        // let num_rows = (self.height / self.grid.element_height).floor() as u32;

        let absolute_position = self.position * render_ctx.screen_size;
        let absolute_size = self.size * render_ctx.screen_size;

        let column_width = absolute_size.x / self.columns as f32;
        for i in 0..(self.columns + 1) {
            let x = (i as f32) * column_width + absolute_position.x;
            let y_start = absolute_position.y;
            let y_end = absolute_position.y + absolute_size.y;
            draw_line(x, y_start, x, y_end, 2.0, WHITE);
        }
        let row_height = absolute_size.y / self.rows as f32;
        for i in 0..(self.columns + 1) {
            let y = (i as f32) * row_height + absolute_position.y;
            let x_start = absolute_position.x;
            let x_end = absolute_size.x + absolute_position.x;
            draw_line(x_start, y, x_end, y, 2.0, WHITE);
        }
        Ok(())
    }
}
