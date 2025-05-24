use std::cell::RefCell;

use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;

use crate::engine::errors::GameResult;
use crate::render::Render;

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
    // fn update_position(&self, new_position: Vec2) {
    //     let current = self.as_displayed().get();
    //     self.as_displayed().set(DisplayedAudioNode {
    //         center: new_position,
    //         ..current
    //     })
    // }
    //
    // fn maybe_start_dragging(&self, cursor: &Vec2) {
    //     let current = self.as_displayed().get();
    //     if self.as_displayed().get().inside(cursor) {}
    // }
    //
    // fn maybe_drag(&self, cursor: Vec2) {
    //     if self.is_cursor_on(&cursor) {
    //         self.update_position(cursor);
    //     }
    // }
}

impl<T> Render for T
where
    T: AudioNode,
{
    fn render(&self) -> GameResult<()> {
        self.as_displayed().borrow().render()?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct DisplayedAudioNode {
    center: Vec2,
    size: Vec2,
    color: Color,
    is_dragged: bool,
}

impl DisplayedAudioNode {
    pub fn new(center: Vec2, size: Vec2, color: Color) -> Self {
        DisplayedAudioNode {
            center,
            size,
            color,
            is_dragged: false,
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
    fn render(&self) -> GameResult<()> {
        draw_rectangle(
            self.center.x - self.size.x / 2.0,
            self.center.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
            self.color,
        );
        Ok(())
    }
}
