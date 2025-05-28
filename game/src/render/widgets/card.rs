use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;

use crate::engine::errors::GameResult;
use crate::render::hover::is_inside;
use crate::render::hover::Hover;
use crate::render::shapes::Shape;
use crate::render::Render;
use crate::render::RenderCtx;

const MARGIN_PERSENTAGE: f32 = 0.2; // TODO: move to config/constants

#[derive(Clone, Copy)]
pub struct Card {
    pub center: Vec2,
    pub size: Vec2,
    pub background_color: Color,
    pub foreground_color: Color,
    shape: Shape,
    is_dragged: bool,
}

impl Card {
    pub fn new(
        center: Vec2,
        size: Vec2,
        background_color: Color,
        foreground_color: Color,
        shape: Shape,
    ) -> Card {
        Card {
            center,
            size,
            background_color,
            foreground_color,
            shape,
            is_dragged: false,
        }
    }
    pub fn start_dragging(&mut self) {
        self.is_dragged = true;
    }
    pub fn stop_dragging(&mut self) {
        self.is_dragged = false;
    }
    pub fn update_dragged_position(&mut self, new_position: Vec2) {
        if self.is_dragged {
            self.center = new_position;
        }
    }

    pub fn snap(&mut self, position: Vec2, margins: Vec2) {
        if is_inside(position - margins, position + margins, self.center) {
            self.center = position;
        }
    }
    pub fn is_dragged(&self) -> bool {
        self.is_dragged
    }
}

impl Render for Card {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        let absolute_center = self.center * render_ctx.screen_size;
        let absolute_size = self.size * render_ctx.screen_size;
        let absolute_margin = absolute_size * MARGIN_PERSENTAGE;
        let absolute_top_left = absolute_center - absolute_size / 2.0;

        draw_rectangle(
            absolute_top_left.x,
            absolute_top_left.y,
            absolute_size.x,
            absolute_size.y,
            self.background_color,
        );

        self.shape.draw(
            render_ctx,
            absolute_top_left + absolute_margin / 2.0,
            absolute_size * (1.0 - MARGIN_PERSENTAGE),
            self.foreground_color, // self.color,
        )?;
        Ok(())
    }
}

impl Hover for Card {
    fn is_hovered_over(&self, relative_mouse_position: Vec2) -> bool {
        is_inside(
            self.center - self.size / 2.0,
            self.center + self.size / 2.0,
            relative_mouse_position,
        )
    }
}
