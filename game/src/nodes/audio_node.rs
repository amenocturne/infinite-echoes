use std::cell::RefCell;

use macroquad::color::Color;
use macroquad::math::{vec2, Vec2};
use macroquad::shapes::draw_rectangle;

use crate::engine::errors::GameResult;
use crate::render::shapes::Shape;
use crate::render::{Render, RenderCtx};

const MARGIN_PERSENTAGE: f32 = 0.2;

pub trait AudioNode {
    fn as_displayed(&self) -> &RefCell<DisplayedAudioNode>;

    fn get_center(&self) -> Vec2 {
        self.as_displayed().borrow().center
    }

    fn is_cursor_on(&self, cursor: &Vec2) -> bool {
        self.as_displayed().borrow().inside(cursor)
    }

    fn maybe_start_dragging(&self, cursor: &Vec2) {
        if self.as_displayed().borrow().inside(cursor) {
            self.as_displayed().borrow_mut().is_dragged = true;
        }
    }
    fn update_dragged_position(&self, cursor: &Vec2) {
        if self.as_displayed().borrow().is_dragged {
            self.as_displayed().borrow_mut().center = *cursor;
        }
    }
    fn stop_dragging(&self) {
        self.as_displayed().borrow_mut().is_dragged = false;
    }
}

impl<T> Render for T
where
    T: AudioNode,
{
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        self.as_displayed().borrow().render(render_ctx)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct DisplayedAudioNode {
    center: Vec2,
    size: Vec2,
    foreground_color: Color,
    background_color: Color,
    is_dragged: bool,
    shape: Shape,
}

impl DisplayedAudioNode {
    pub fn new(
        center: Vec2,
        size: Vec2,
        foreground_color: Color,
        background_color: Color,
        shape: Shape,
    ) -> Self {
        DisplayedAudioNode {
            center,
            size,
            foreground_color,
            background_color,
            is_dragged: false,
            shape,
        }
    }
    pub fn inside(&self, cursor: &Vec2) -> bool {
        (self.center.x - self.size.x / 2.0) < cursor.x
            && cursor.x < (self.center.x + self.size.x / 2.0)
            && (self.center.y - self.size.y / 2.0) < cursor.y
            && cursor.y < (self.center.y + self.size.y / 2.0)
    }

    pub fn start_dragging(&mut self) {
        self.is_dragged = true;
    }
    pub fn stop_dragging(&mut self) {
        self.is_dragged = false;
    }
}

impl Render for DisplayedAudioNode {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        let margin_x = self.size.x * MARGIN_PERSENTAGE;
        let margin_y = self.size.y * MARGIN_PERSENTAGE;

        let top_left_x = self.center.x - self.size.x / 2.0;
        let top_left_y = self.center.y - self.size.y / 2.0;

        draw_rectangle(
            top_left_x,
            top_left_y,
            self.size.x,
            self.size.y,
            self.background_color,
        );

        self.shape.draw(
            render_ctx,
            vec2(top_left_x + margin_x / 2.0, top_left_y + margin_y / 2.0),
            vec2(
                self.size.x * (1.0 - MARGIN_PERSENTAGE),
                self.size.y * (1.0 - MARGIN_PERSENTAGE),
            ),
            self.foreground_color, // self.color,
        )?;

        Ok(())
    }
}
