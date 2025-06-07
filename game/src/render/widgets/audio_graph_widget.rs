use std::cell::RefCell;

use macroquad::math::vec2;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;
use crate::nodes::AudioNodeType;
use crate::render::draggable_card_buffer::DraggableCardBuffer;
use crate::render::rectangle_boundary::RectangleBoundary;
use crate::render::Render;
use crate::render::RenderCtx;

use super::card_widget::{Card, CardType};
use super::grid_widget::GridWidget;

pub struct AudioGraphWidget {
    center: Vec2,
    size: Vec2,
    cards: Vec<RefCell<Card>>,
    card_size: Vec2,
    grid: GridWidget,
}

impl AudioGraphWidget {
    pub fn new(center: Vec2, size: Vec2, card_size: Vec2) -> Self {
        let grid = GridWidget::new(center, size, 0, 1);
        let mut result = Self {
            center,
            size,
            cards: vec![],
            card_size,
            grid,
        };
        result.organize_cards();
        result
    }

    fn update_grid(&mut self) {
        self.grid = GridWidget::new(self.center, self.size, self.cards.len() as u32, 1);
    }
}

impl DraggableCardBuffer for AudioGraphWidget {
    fn cards(&self) -> &Vec<RefCell<Card>> {
        &self.cards
    }

    fn push_card(&mut self, card: RefCell<Card>) {
        self.cards.push(card)
    }

    fn remove_card(&mut self, i: usize) -> RefCell<Card> {
        self.cards.remove(i)
    }

    fn insert_card(&mut self, i: usize, card: RefCell<Card>) {
        self.cards.insert(i, card);
    }

    fn card_centers(&self) -> Vec<Vec2> {
        self.grid_centers(self.grid.columns(), self.grid.rows())
    }

    fn snapping_margin(&self) -> Vec2 {
        self.card_size / 2.0
    }

    fn drag_in_regions(&self, node_type: AudioNodeType) -> Vec<(usize, Vec2, Vec2)> {
        let mut allowed_places = vec![];
        let mut maybe_before = None;

        for (i, after) in self.cards.iter().enumerate() {
            if node_type.can_put_between_loose(&maybe_before, &Some(after.borrow().as_type())) {
                allowed_places.push((i, after.borrow().center()));
            }
            maybe_before = Some(after.borrow().as_type());
        }
        if node_type.can_put_between_loose(&maybe_before, &None) {
            allowed_places.push((self.cards.len(), self.right_center()));
        }

        let box_size = self.grid.single_cell_size();
        let mut prev_top_left = self.top_left();
        let mut regions = vec![];

        for (i, c) in allowed_places.iter() {
            regions.push((*i, prev_top_left, *c + vec2(0.0, box_size.y / 2.0)));
            prev_top_left = *c - vec2(0.0, box_size.y / 2.0);
        }
        if let Some((i, _)) = allowed_places.last() {
            regions.push((*i, prev_top_left, self.bottom_right()));
        }

        regions
    }

    fn organize_cards(&mut self) {
        self.update_grid();
        let centers = self.grid_centers(self.grid.columns(), self.grid.rows());
        _ = self
            .cards
            .iter()
            .zip(centers)
            .map(|(card, center)| {
                card.borrow_mut().center = center;
            })
            .collect::<Vec<()>>();
    }

    fn set_cards(&mut self, card_types: Vec<CardType>) {
        use macroquad::color::{BLACK, WHITE};
        self.cards = card_types
            .iter()
            .map(|t| {
                RefCell::new(Card::new(
                    vec2(0.0, 0.0),
                    self.card_size,
                    WHITE,
                    BLACK,
                    *t,
                ))
            })
            .collect();
        self.organize_cards();
    }
}

impl RectangleBoundary for AudioGraphWidget {
    fn center(&self) -> Vec2 {
        self.center
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

impl Render for AudioGraphWidget {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        self.grid.render(render_ctx)?;
        for c in &self.cards {
            c.borrow().render(render_ctx)?;
        }
        Ok(())
    }
}
