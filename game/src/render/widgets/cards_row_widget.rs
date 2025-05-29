use std::cell::RefCell;

use macroquad::color::BLACK;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;
use crate::render::hover::Hover;
use crate::render::rectangle_boundary::RectangleBoundary;
use crate::render::Render;
use crate::render::RenderCtx;

use super::card::Card;
use super::card::CardType;
use super::grid::GridWidget;

pub struct CardsRowWidget {
    center: Vec2,
    size: Vec2,
    cards: Vec<RefCell<Card>>,
    grid: GridWidget,
}

impl CardsRowWidget {
    pub fn new(center: Vec2, size: Vec2, card_size: Vec2, card_margin: f32) -> Self {
        let max_cards = (size / (card_size + card_margin)).x.floor() as u32;

        let card_centers = Self::grid_centers_from(center, size, max_cards, 1);

        let cards = card_centers
            .iter()
            .map(|c| {
                RefCell::new(Card::new(
                    c.clone(),
                    card_size,
                    WHITE,
                    BLACK,
                    CardType::AudioEffect,
                ))
            })
            .collect();

        let grid = GridWidget::new(center, size, max_cards, 1);
        Self {
            center,
            size,
            cards,
            grid,
        }
    }

    fn card_centers(&self) -> Vec<Vec2> {
        self.grid_centers(self.cards.len() as u32, 1)
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

impl RectangleBoundary for CardsRowWidget {
    fn center(&self) -> Vec2 {
        self.center
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

impl Render for CardsRowWidget {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        self.grid.render(render_ctx)?;
        for c in &self.cards {
            c.borrow().render(render_ctx)?;
        }
        Ok(())
    }
}
