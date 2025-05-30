use super::line_widget::LineWidget;
use crate::engine::errors::GameResult;
use crate::render::rectangle_boundary::RectangleBoundary;
use crate::render::widgets::rectangle_widget::RectangleWidget;
use crate::render::Render;
use crate::render::RenderCtx;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::math::Vec2;

/// Position and size are in percent to the whole screen width and height
#[derive(Debug)]
pub struct GridWidget {
    center: Vec2,
    size: Vec2,
    columns: u32,
    rows: u32,
}

impl GridWidget {
    pub fn new(center: Vec2, size: Vec2, columns: u32, rows: u32) -> Self {
        GridWidget {
            center,
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
                    cell_size.x / 2.0 + col as f32 * cell_size.x + self.top_left().x,
                    cell_size.y / 2.0 + row as f32 * cell_size.y + self.top_left().y,
                ))
            }
        }
        return points;
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }
    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn single_cell_size(&self) -> Vec2 {
        self.size / vec2(self.columns as f32, self.rows as f32)
    }
}

impl RectangleBoundary for GridWidget {
    fn center(&self) -> Vec2 {
        self.center
    }
    fn size(&self) -> Vec2 {
        self.size
    }
}

impl Render for GridWidget {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        let cell_size = self.size / vec2(self.columns as f32, self.rows as f32);
        let boundary = RectangleWidget::with_boundary(self.center, self.size, None, 2.0, WHITE);
        vec![
            LineWidget::new(self.top_left(), self.top_right(), 2.0, WHITE),
            LineWidget::new(self.top_right(), self.bottom_right(), 2.0, WHITE),
            LineWidget::new(self.bottom_right(), self.bottom_left(), 2.0, WHITE),
            LineWidget::new(self.bottom_left(), self.top_left(), 2.0, WHITE),
        ];
        let columns = (1..(self.columns)).map(|i| {
            LineWidget::new(
                vec2((i as f32) * cell_size.x, 0.0) + self.left_center(),
                vec2((i as f32) * cell_size.x, self.size.y) + self.left_center(),
                2.0,
                WHITE,
            )
        });

        let rows = (1..(self.rows)).map(|i| {
            LineWidget::new(
                vec2(0.0, (i as f32) * cell_size.y) + self.left_center(),
                vec2(self.size.x, (i as f32) * cell_size.y) + self.left_center(),
                2.0,
                WHITE,
            )
        });

        boundary.render(render_ctx)?;

        columns
            .chain(rows)
            .map(|t| t.render(render_ctx))
            .collect::<GameResult<()>>()
    }
}
