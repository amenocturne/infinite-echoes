use std::cell::RefCell;

use macroquad::color::BLACK;
use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;
use crate::nodes::audio_graph::AudioGraph;
use crate::render::draggable_card_buffer::DraggableCardBuffer;
use crate::render::rectangle_boundary::RectangleBoundary;
use crate::render::Render;
use crate::render::RenderCtx;

use super::card_widget::Card;
use super::grid_widget::GridWidget;

pub struct AudioGraphWidget {
    center: Vec2,
    size: Vec2,
    cards: Vec<RefCell<Card>>,
    card_size: Vec2,
    grid: GridWidget,
}

impl AudioGraphWidget {
    pub fn new(center: Vec2, size: Vec2, card_size: Vec2, audio_graph: &AudioGraph) -> Self {
        let card_types = audio_graph.as_card_types();
        let default_pos = vec2(0.0, 0.0);

        let cards: Vec<RefCell<Card>> = card_types
            .iter()
            .map(|card_type| {
                RefCell::new(Card::new(
                    default_pos,
                    card_size,
                    WHITE,
                    BLACK,
                    card_type.clone(),
                ))
            })
            .collect();

        let grid = GridWidget::new(center, size, cards.len() as u32, 1);
        let mut result = Self {
            center,
            size,
            cards,
            card_size,
            grid,
        };
        result.organize_cards();
        result
    }

    pub fn update_audio_graph(&mut self, audio_graph: &AudioGraph) {
        let card_types = audio_graph.as_card_types();
        let cards = card_types
            .iter()
            .map(|card_type| {
                RefCell::new(Card::new(
                    vec2(0.0, 0.0),
                    self.card_size,
                    WHITE,
                    BLACK,
                    card_type.clone(),
                ))
            })
            .collect();

        self.cards = cards;
        self.organize_cards();
    }

    fn update_grid(&mut self) {
        self.grid = GridWidget::new(self.center, self.size, self.cards.len() as u32, 1);
    }
}

impl DraggableCardBuffer for AudioGraphWidget {
    fn cards(&self) -> &Vec<RefCell<Card>> {
        &self.cards
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

    fn drag_in_regions(&self) -> Vec<(Vec2, Vec2)> {
        let box_size = self.grid.single_cell_size();
        let shift = -vec2(box_size.x, 0.0) / 2.0;
        let mut regions: Vec<_> = self
            .card_centers()
            .into_iter()
            .map(|c| (c - box_size / 2.0 + shift, c + box_size / 2.0 + shift))
            .collect();

        if let Some(last) = self.card_centers().last() {
            let top_left = *last - vec2(0.0, box_size.y / 2.0);
            let bottom_right = self.bottom_right();
            regions.push((top_left, bottom_right))
        } else {
            regions.push((self.top_left(), self.bottom_right()))
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
