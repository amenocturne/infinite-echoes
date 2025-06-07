use std::cell::RefCell;

use macroquad::color::BLACK;
use macroquad::math::vec2;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;
use crate::engine::game_config::CardColorConfig;
use crate::nodes::AudioNodeType;
use crate::render::draggable_card_buffer::DraggableCardBuffer;
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
    card_colors: CardColorConfig,
}

impl CardsRowWidget {
    pub fn new(
        center: Vec2,
        size: Vec2,
        card_size: Vec2,
        card_types: Vec<CardType>,
        card_colors: CardColorConfig,
    ) -> Self {
        let default_pos = vec2(0.0, 0.0);

        let cards: Vec<RefCell<Card>> = card_types
            .iter()
            .map(|t| {
                RefCell::new(Card::new(
                    default_pos,
                    card_size,
                    t.get_color(&card_colors),
                    BLACK,
                    t.clone(),
                ))
            })
            .collect();

        let grid = GridWidget::new(center, size, cards.len() as u32, 1);
        let mut res = Self {
            center,
            size,
            cards,
            card_size,
            grid,
            card_colors,
        };
        res.organize_cards();
        res
    }

    fn update_grid(&mut self) {
        self.grid = GridWidget::new(self.center, self.size, self.cards.len() as u32, 1);
    }
}

impl DraggableCardBuffer for CardsRowWidget {
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

    fn drag_in_regions(&self, _node_type: AudioNodeType) -> Vec<(usize, Vec2, Vec2)> {
        let box_size = self.grid.single_cell_size();
        let mut prev_top_left = self.top_left();
        let mut regions = vec![];
        let actual_card_centers: Vec<_> =
            self.cards().iter().map(|c| c.borrow().center()).collect();

        for (i, c) in actual_card_centers.iter().enumerate() {
            regions.push((i, prev_top_left, *c + vec2(0.0, box_size.y / 2.0)));
            prev_top_left = *c - vec2(0.0, box_size.y / 2.0);
        }
        regions.push((
            actual_card_centers.len(),
            prev_top_left,
            self.bottom_right(),
        ));
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
                card.borrow_mut().center = center.clone();
            })
            .collect::<Vec<()>>();
    }

    fn set_cards(&mut self, card_types: Vec<CardType>) {
        self.cards = card_types
            .iter()
            .map(|t| {
                RefCell::new(Card::new(
                    vec2(0.0, 0.0),
                    self.card_size,
                    t.get_color(&self.card_colors),
                    BLACK,
                    *t,
                ))
            })
            .collect();
        self.organize_cards();
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
