use std::cell::RefCell;

use macroquad::color::BLACK;
use macroquad::color::WHITE;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;
use crate::nodes::audio_graph::AudioGraph;
use crate::render::rectangle_boundary::RectangleBoundary;
use crate::render::Render;
use crate::render::RenderCtx;

use super::card::Card;
use super::card::CardType;
use super::grid::GridWidget;

pub struct AudioGraphWidget {
    center: Vec2,
    size: Vec2,
    cards: Vec<RefCell<Card>>,
    max_cards: u32,
    grid: GridWidget,
}

impl AudioGraphWidget {
    pub fn new(
        center: Vec2,
        size: Vec2,
        card_size: Vec2,
        card_margin: f32,
        audio_graph: &AudioGraph,
    ) -> Self {
        let max_cards = (size / (card_size + card_margin)).x.floor() as u32;

        let card_shapes = vec![
            vec![
                CardType::NoteGenerator,  // note_generator
                CardType::SineOscillator, // oscillator
            ],
            audio_graph
                .audio_effects
                .iter()
                .map(|_| CardType::AudioEffect)
                .collect(),
        ]
        .concat();

        let card_centers = Self::grid_centers_from(center, size, max_cards, 1);
        let cards = card_shapes
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

        let grid = GridWidget::new(center, size, max_cards, 1);
        Self {
            center,
            size,
            cards,
            max_cards,
            grid,
        }
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
