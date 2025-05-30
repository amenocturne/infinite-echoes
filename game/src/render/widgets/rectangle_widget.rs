use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;

use crate::{
    engine::errors::GameResult,
    render::{rectangle_boundary::RectangleBoundary, Render, RenderCtx},
};

use super::line_widget::LineWidget;

#[derive(Clone)]
struct Boundary {
    boundary_thickness: f32,
    boundary_color: Color,
}

pub struct RectangleWidget {
    center: Vec2,
    size: Vec2,
    fill_color: Option<Color>,
    boundary: Option<Boundary>,
}

impl RectangleWidget {
    pub fn with_boundary(
        center: Vec2,
        size: Vec2,
        fill_color: Option<Color>,
        boundary_thickness: f32,
        boundary_color: Color,
    ) -> Self {
        Self {
            center,
            size,
            fill_color,
            boundary: Some(Boundary {
                boundary_thickness,
                boundary_color,
            }),
        }
    }

    pub fn no_boundary(center: Vec2, size: Vec2, fill_color: Option<Color>) -> Self {
        Self {
            center,
            size,
            fill_color,
            boundary: None,
        }
    }
}

impl RectangleBoundary for RectangleWidget {
    fn center(&self) -> Vec2 {
        self.center
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

impl Render for RectangleWidget {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        let absolute_top_left = self.top_left() * render_ctx.screen_size;
        let absolute_size = self.size() * render_ctx.screen_size;

        self.fill_color.map(|c| {
            draw_rectangle(
                absolute_top_left.x,
                absolute_top_left.y,
                absolute_size.x,
                absolute_size.y,
                c,
            )
        });

        self.boundary
            .as_ref()
            .map(|b| {
                let make_line = |start: Vec2, end: Vec2| {
                    LineWidget::new(start, end, b.boundary_thickness, b.boundary_color)
                };

                vec![
                    make_line(self.top_left(), self.top_right()),
                    make_line(self.top_right(), self.bottom_right()),
                    make_line(self.bottom_right(), self.bottom_left()),
                    make_line(self.bottom_left(), self.top_left()),
                ]
            })
            .unwrap_or(vec![])
            .iter()
            .map(|b| b.render(render_ctx))
            .collect::<GameResult<()>>()
    }
}
