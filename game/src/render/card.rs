use macroquad::{color::Color, math::Vec2, shapes::draw_rectangle};

use crate::engine::errors::GameResult;

use super::{
    hover::{is_inside, Hover},
    Render, RenderCtx,
};

#[derive(Clone, Copy)]
pub struct Card {
    pub center: Vec2,
    pub size: Vec2,
    pub color: Color,
    is_dragged: bool,
}

impl Card {
    pub fn new(center: Vec2, size: Vec2, color: Color) -> Card {
        Card {
            center,
            size,
            color,
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
        draw_rectangle(
            absolute_center.x - absolute_size.x / 2.0,
            absolute_center.y - absolute_size.y / 2.0,
            absolute_size.x,
            absolute_size.y,
            self.color,
        );
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
