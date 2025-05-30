use std::cell::RefCell;

use macroquad::color::BLACK;
use macroquad::color::WHITE;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;
use crate::nodes::audio_graph::AudioGraph;
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
        let card_centers = Self::grid_centers_from(center, size, card_types.len() as u32, 1);
        let cards = card_types
            .iter()
            .zip(card_centers)
            .map(|(card_shape, card_center)| {
                RefCell::new(Card::new(
                    card_center,
                    card_size,
                    WHITE,
                    BLACK,
                    card_shape.clone(),
                ))
            })
            .collect();

        let grid = GridWidget::new(center, size, card_types.len() as u32, 1);
        Self {
            center,
            size,
            cards,
            card_size,
            grid,
        }
    }
    pub fn update_audio_graph(&mut self, audio_graph: &AudioGraph) {
        let card_types = audio_graph.as_card_types();
        let cards = card_types
            .iter()
            .zip(self.card_centers())
            .map(|(card_shape, card_center)| {
                RefCell::new(Card::new(
                    card_center,
                    self.card_size,
                    WHITE,
                    BLACK,
                    card_shape.clone(),
                ))
            })
            .collect();

        let grid = GridWidget::new(self.center, self.size, card_types.len() as u32, 1);
        self.cards = cards;
        self.grid = grid;
    }


    fn card_centers(&self) -> Vec<Vec2> {
        Self::grid_centers_from(self.center, self.size, self.cards.len() as u32, 1)
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
