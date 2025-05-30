use std::cell::RefCell;

use macroquad::color::BLACK;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::math::Vec2;
use miniquad::info;

use crate::engine::errors::GameResult;
use crate::render::hover::Hover;
use crate::render::rectangle_boundary::RectangleBoundary;
use crate::render::Render;
use crate::render::RenderCtx;

use super::card_widget::Card;
use super::card_widget::CardType;
use super::grid_widget::GridWidget;

#[derive(Debug)]
pub struct CardsRowWidget {
    center: Vec2,
    size: Vec2,
    cards: Vec<RefCell<Card>>,
    card_size: Vec2,
    grid: GridWidget,
}

impl CardsRowWidget {
    pub fn new(center: Vec2, size: Vec2, card_size: Vec2, card_types: Vec<CardType>) -> Self {
        // let max_cards = (size / (card_size + card_margin)).x.floor() as u32;
        let default_pos = vec2(0.0, 0.0);

        let cards: Vec<RefCell<Card>> = card_types
            .iter()
            .map(|t| RefCell::new(Card::new(default_pos, card_size, WHITE, BLACK, t.clone())))
            .collect();

        let grid = GridWidget::new(center, size, cards.len() as u32, 1);
        let mut res = Self {
            center,
            size,
            cards,
            card_size,
            grid,
        };
        res.move_cards_to_default_positions();
        res
    }

    fn card_centers(&self) -> Vec<Vec2> {
        self.grid_centers(self.grid.columns(), self.grid.rows())
    }

    pub fn start_dragging(&self, mouse_position: Vec2) {
        for c in &self.cards {
            if c.borrow().is_hovered_over(mouse_position) {
                c.borrow_mut().start_dragging();
                return;
            }
        }
    }

    // Removes and returns dragged card
    pub fn stop_dragging(&mut self) -> Option<RefCell<Card>> {
        let mut removed = None;
        let mut removed_index = None;
        for (i, c) in self.cards.iter().enumerate() {
            if c.borrow().is_dragged() {
                c.borrow_mut().stop_dragging();
                removed_index = Some(i);
                removed = Some(c.clone());
                break;
            }
        }
        removed_index.map(|i| self.cards.remove(i));
        self.move_cards_to_default_positions();
        removed
    }

    // stops dragging on all cards and moves them to default locations
    pub fn abort_dragging(&mut self) {
        for c in &self.cards {
            if c.borrow().is_dragged() {
                c.borrow_mut().stop_dragging();
                self.move_cards_to_default_positions();
                break;
            }
        }
    }

    fn move_cards_to_default_positions(&mut self) {
        self.update_grid();
        let centers = self.grid_centers(self.grid.columns(), self.grid.rows());
        _ = self
            .cards
            .iter()
            .zip(centers)
            .map(|(card, center)| {
                card.borrow_mut().center = center.clone();
            })
            .collect::<Vec<()>>();
    }

    fn update_grid(&mut self) {
        self.grid = GridWidget::new(self.center, self.size, self.cards.len() as u32, 1);
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

    pub fn add_card(&mut self, card: &RefCell<Card>) -> bool {
        let box_size = self.grid.single_cell_size();
        let shift = -vec2(box_size.x, 0.0) / 2.0;
        let add_boxes = self
            .card_centers()
            .into_iter()
            .map(|c| (c - box_size / 2.0 + shift, c + box_size / 2.0 + shift));
        for (i, (top_left, bottom_right)) in add_boxes.enumerate() {
            if Self::is_inside_from(top_left, bottom_right, card.borrow().center()) {
                self.cards.insert(i, card.clone());
                self.move_cards_to_default_positions();
                return true;
            }
        }
        if let Some(last) = self.card_centers().last() {
            let top_left = *last - vec2(0.0, box_size.y / 2.0);
            let bottom_right = self.bottom_right();
            if Self::is_inside_from(top_left, bottom_right, card.borrow().center()) {
                self.cards.push(card.clone());
                self.move_cards_to_default_positions();
                return true;
            }
        }
        false
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
