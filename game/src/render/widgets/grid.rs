use super::line::LineWidget;
use crate::engine::errors::GameResult;
use crate::render::Render;
use crate::render::RenderCtx;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::math::Vec2;

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
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        let cell_size = self.size / vec2(self.columns as f32, self.rows as f32);
        let columns = (0..(self.columns + 1)).map(|i| {
            LineWidget::new(
                vec2((i as f32) * cell_size.x, 0.0) + self.position,
                vec2((i as f32) * cell_size.x, self.size.y) + self.position,
                2.0,
                WHITE,
            )
        });

        let rows = (0..(self.rows + 1)).map(|i| {
            LineWidget::new(
                vec2(0.0, (i as f32) * cell_size.y) + self.position,
                vec2(self.size.x, (i as f32) * cell_size.y) + self.position,
                2.0,
                WHITE,
            )
        });

        columns
            .chain(rows)
            .map(|t| t.render(render_ctx))
            .collect::<GameResult<()>>()
    }
}
