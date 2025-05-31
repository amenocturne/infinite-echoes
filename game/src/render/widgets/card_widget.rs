use macroquad::color::Color;
use macroquad::color::GRAY;
use macroquad::math::Vec2;

use crate::engine::errors::GameResult;
use crate::nodes::AudioNodeType;
use crate::render::hover::Hover;
use crate::render::rectangle_boundary::RectangleBoundary;
use crate::render::shapes::Shape;
use crate::render::Render;
use crate::render::RenderCtx;

use super::rectangle_widget::RectangleWidget;

const MARGIN_PERSENTAGE: f32 = 0.2; // TODO: move to config/constants

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub center: Vec2,
    pub size: Vec2,
    pub background_color: Color,
    pub foreground_color: Color,
    card_type: CardType,
    is_dragged: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum CardType {
    NoteGenerator,
    SineOscillator,
    SquareOscilaltor,
    AudioEffect,
    NoteEffect,
}

impl CardType {
    pub fn as_shape(&self) -> Shape {
        match self {
            CardType::AudioEffect => Shape::Blank,
            CardType::NoteEffect => Shape::Blank,
            CardType::SineOscillator => Shape::SineWave,
            CardType::SquareOscilaltor => Shape::SquareWave,
            CardType::NoteGenerator => Shape::Piano,
        }
    }
    pub fn as_type(&self) -> AudioNodeType {
        match self {
            CardType::AudioEffect => AudioNodeType::AudioEffect,
            CardType::NoteEffect => AudioNodeType::NoteEffect,
            CardType::SineOscillator => AudioNodeType::Oscillator,
            CardType::SquareOscilaltor => AudioNodeType::Oscillator,
            CardType::NoteGenerator => AudioNodeType::NoteGenerator,
        }
    }
}

impl Card {
    pub fn new(
        center: Vec2,
        size: Vec2,
        background_color: Color,
        foreground_color: Color,
        card_type: CardType,
    ) -> Card {
        Card {
            center,
            size,
            background_color,
            foreground_color,
            card_type,
            is_dragged: false,
        }
    }
    pub fn start_dragging(&mut self) {
        self.is_dragged = true;
    }
    pub fn stop_dragging(&mut self) {
        self.is_dragged = false;
    }
    pub fn update_dragged_position(&mut self, new_position: Vec2) {
        if self.is_dragged {
            self.center = new_position;
        }
    }

    pub fn snap(&mut self, position: Vec2, margins: Vec2) {
        if Self::is_inside_from(position - margins, position + margins, self.center) {
            self.center = position;
        }
    }
    pub fn is_dragged(&self) -> bool {
        self.is_dragged
    }

    pub fn as_type(&self) -> AudioNodeType {
        self.card_type.as_type()
    }

    pub fn card_type(&self) -> CardType {
        self.card_type
    }
}

impl RectangleBoundary for Card {
    fn center(&self) -> Vec2 {
        self.center
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

impl Render for Card {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        let rect = RectangleWidget::with_boundary(
            self.center,
            self.size,
            Some(self.background_color),
            5.0,
            GRAY,
        );

        let absolute_center = self.center * render_ctx.screen_size;
        let absolute_size = self.size * render_ctx.screen_size;
        let absolute_margin = absolute_size * MARGIN_PERSENTAGE;
        let absolute_top_left = absolute_center - absolute_size / 2.0;

        rect.render(render_ctx)?;
        self.card_type.as_shape().draw(
            render_ctx,
            absolute_top_left + absolute_margin / 2.0,
            absolute_size * (1.0 - MARGIN_PERSENTAGE),
            self.foreground_color, // self.color,
        )?;
        Ok(())
    }
}

impl Hover for Card {
    fn is_hovered_over(&self, relative_mouse_position: Vec2) -> bool {
        Self::is_inside_from(
            self.center - self.size / 2.0,
            self.center + self.size / 2.0,
            relative_mouse_position,
        )
    }
}
