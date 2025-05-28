use std::cell::RefCell;

use macroquad::color::BLACK;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;
use crate::render::hover::Hover;
use crate::render::shapes::Shape;
use crate::render::Render;
use crate::render::RenderCtx;

use super::card::Card;
use super::grid::GridWidget;

pub struct CardsRow {
    center: Vec2,
    size: Vec2,
    cards: Vec<RefCell<Card>>,
    grid: GridWidget,
}

impl CardsRow {
    pub fn new(num_cards: u32, center: Vec2, size: Vec2, card_scale: f32) -> Self {
        let mut cards = vec![];
        let card_size = size / vec2(num_cards as f32, 1.0);
        for i in 0..num_cards {
            let card_center = vec2(
                (center.x - size.x / 2.0 + card_size.x / 2.0) + i as f32 * card_size.x,
                center.y,
            );
            cards.push(RefCell::new(Card::new(
                card_center,
                card_size * card_scale,
                WHITE,
                BLACK,
                Shape::SineWave,
            )))
        }
        let grid = GridWidget::new(center - size / 2.0, size, num_cards, 1);
        Self {
            center,
            size,
            cards,
            grid,
        }
    }

    fn card_centers(&self) -> Vec<Vec2> {
        let mut card_centers = vec![];
        let card_size = self.size / vec2(self.cards.len() as f32, 1.0);
        for i in 0..self.cards.len() {
            card_centers.push(vec2(
                (self.center.x - self.size.x / 2.0 + card_size.x / 2.0) + i as f32 * card_size.x,
                self.center.y,
            ))
        }
        return card_centers;
    }

    pub fn start_dragging(&self, mouse_position: Vec2) {
        // prevents dragging multiple cards
        for c in &self.cards {
            if c.borrow().is_dragged() {
                return;
            }
        }

        for c in &self.cards {
            if c.borrow().is_hovered_over(mouse_position) {
                c.borrow_mut().start_dragging();
                return; // prevents dragging multiple cards
            }
        }
    }

    pub fn stop_dragging(&self) {
        for c in &self.cards {
            c.borrow_mut().stop_dragging();
        }
    }

    pub fn update_dragged_position(&self, mouse_position: Vec2) {
        for c in &self.cards {
            c.borrow_mut().update_dragged_position(mouse_position);
        }
    }

    pub fn snap(&self, snapping_margin: f32) {
        let card_size = self.size / vec2(self.cards.len() as f32, 1.0);
        for c in &self.cards {
            for center in self.card_centers() {
                c.borrow_mut().snap(center, card_size * snapping_margin);
            }
        }
    }
}

impl Render for CardsRow {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        self.grid.render(render_ctx)?;
        for c in &self.cards {
            c.borrow().render(render_ctx)?;
        }
        Ok(())
    }
}
