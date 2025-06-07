use macroquad::color::Color;
use macroquad::color::GRAY;
use macroquad::math::Vec2;
use macroquad::text::{draw_text, measure_text};
use serde::{Deserialize, Serialize};

use crate::engine::errors::GameResult;
use crate::engine::game_config::CardColorConfig;
use crate::nodes::audio_effect::FilterType;
use crate::nodes::note_effect::ChangeLenType;
use crate::nodes::note_effect::ScaleType;
use crate::nodes::note_generator::NoteName;
use crate::nodes::oscillator::WaveShape;
use crate::nodes::AudioNodeType;
use crate::render::hover::Hover;
use crate::render::rectangle_boundary::RectangleBoundary;
use crate::render::shapes::Shape;
use crate::render::Render;
use crate::render::RenderCtx;

use super::rectangle_widget::RectangleWidget;

const MARGIN_PERSENTAGE: f32 = 0.2;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub center: Vec2,
    pub size: Vec2,
    pub background_color: Color,
    pub foreground_color: Color,
    card_type: CardType,
    is_dragged: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CardType {
    NoteGenerator(NoteName),
    NoteEffect(NoteName, ScaleType),
    ChangeLen(ChangeLenType),
    Oscillator(WaveShape),
    Filter(FilterType),
    Distortion,
    Reverb,
}

impl CardType {
    /// Converts a CardType to a unique u16 identifier
    pub fn to_id(&self) -> u16 {
        match self {
            // NoteGenerator: 0-11 (12 values for each note)
            CardType::NoteGenerator(note) => note.to_int() as u16,

            // NoteEffect: 100-123 (12 notes * 2 scale types = 24 values)
            CardType::NoteEffect(note, scale) => {
                100 + note.to_int() as u16 * 2 + match scale {
                    ScaleType::Major => 0,
                    ScaleType::Minor => 1,
                }
            }

            // ChangeLen: 200-201 (2 values)
            CardType::ChangeLen(change_type) => {
                200 + match change_type {
                    ChangeLenType::Double => 0,
                    ChangeLenType::Half => 1,
                }
            }

            // Oscillator: 300-301 (2 values)
            CardType::Oscillator(wave) => {
                300 + match wave {
                    WaveShape::Sine => 0,
                    WaveShape::Square => 1,
                }
            }

            // Filter: 400-402 (3 values)
            CardType::Filter(filter) => {
                400 + match filter {
                    FilterType::LowPass => 0,
                    FilterType::HighPass => 1,
                    FilterType::Notch => 2,
                }
            }

            // Distortion: 500 (1 value)
            CardType::Distortion => 500,

            // Reverb: 600 (1 value)
            CardType::Reverb => 600,
        }
    }

    /// Converts a u16 identifier back to a CardType
    pub fn from_id(id: u16) -> Option<Self> {
        match id {
            // NoteGenerator: 0-11
            0..=11 => Some(CardType::NoteGenerator(NoteName::from_int(id as u32))),

            // NoteEffect: 100-123
            100..=123 => {
                let note_id = (id - 100) / 2;
                let scale_id = (id - 100) % 2;
                let note = NoteName::from_int(note_id as u32);
                let scale = if scale_id == 0 {
                    ScaleType::Major
                } else {
                    ScaleType::Minor
                };
                Some(CardType::NoteEffect(note, scale))
            }

            // ChangeLen: 200-201
            200 => Some(CardType::ChangeLen(ChangeLenType::Double)),
            201 => Some(CardType::ChangeLen(ChangeLenType::Half)),

            // Oscillator: 300-301
            300 => Some(CardType::Oscillator(WaveShape::Sine)),
            301 => Some(CardType::Oscillator(WaveShape::Square)),

            // Filter: 400-402
            400 => Some(CardType::Filter(FilterType::LowPass)),
            401 => Some(CardType::Filter(FilterType::HighPass)),
            402 => Some(CardType::Filter(FilterType::Notch)),

            // Distortion: 500
            500 => Some(CardType::Distortion),

            // Reverb: 600
            600 => Some(CardType::Reverb),

            // Invalid ID
            _ => None,
        }
    }
}

impl CardType {
    pub fn as_shape(&self) -> Shape {
        match self {
            CardType::NoteGenerator(_) => Shape::NOTE,
            CardType::NoteEffect(_, _) => Shape::CHORD,
            CardType::ChangeLen(ChangeLenType::Half) => Shape::FASTER,
            CardType::ChangeLen(ChangeLenType::Double) => Shape::SLOWER,
            CardType::Oscillator(WaveShape::Sine) => Shape::SINE,
            CardType::Oscillator(WaveShape::Square) => Shape::SQUARE,
            CardType::Filter(FilterType::Notch) => Shape::NOTCH,
            CardType::Filter(FilterType::LowPass) => Shape::LOWPASS,
            CardType::Filter(FilterType::HighPass) => Shape::HIGHPASS,
            CardType::Distortion => Shape::DISTORTION,
            CardType::Reverb => Shape::REVERB,
        }
    }
    pub fn as_type(&self) -> AudioNodeType {
        match self {
            CardType::NoteGenerator(_) => AudioNodeType::NoteGenerator,
            CardType::NoteEffect(_, _) => AudioNodeType::NoteEffect,
            CardType::ChangeLen(_) => AudioNodeType::NoteEffect,
            CardType::Oscillator(_) => AudioNodeType::Oscillator,
            CardType::Filter(_) => AudioNodeType::AudioEffect,
            CardType::Distortion => AudioNodeType::AudioEffect,
            CardType::Reverb => AudioNodeType::AudioEffect,
        }
    }

    pub fn get_label(&self) -> Option<String> {
        match self {
            CardType::NoteGenerator(note_name) => Some(note_name.to_string()),
            CardType::NoteEffect(note_name, scale_type) => {
                let scale_str = match scale_type {
                    ScaleType::Major => "Maj",
                    ScaleType::Minor => "Min",
                };
                Some(format!("{} {}", note_name.to_string(), scale_str))
            }
            CardType::ChangeLen(change_type) => match change_type {
                ChangeLenType::Double => Some("x2".to_string()),
                ChangeLenType::Half => Some("/2".to_string()),
            },
            _ => None,
        }
    }

    pub fn get_note_name(&self) -> Option<NoteName> {
        match self {
            CardType::NoteGenerator(note_name) => Some(*note_name),
            _ => None,
        }
    }

    pub fn get_color(&self, colors: &CardColorConfig) -> Color {
        match self.as_type() {
            AudioNodeType::NoteGenerator => colors.note_generator,
            AudioNodeType::NoteEffect => colors.note_effect,
            AudioNodeType::Oscillator => colors.oscillator,
            AudioNodeType::AudioEffect => colors.audio_effect,
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
            self.foreground_color,
        )?;

        if let Some(label) = self.card_type.get_label() {
            let font_size = absolute_size.y * 0.3;
            let text_dims = measure_text(&label, None, font_size as u16, 1.0);

            let text_x = absolute_center.x - text_dims.width / 2.0;
            // Position text at the bottom of the card with some padding.
            let text_y = absolute_top_left.y + absolute_size.y - (font_size * 0.25);

            draw_text(&label, text_x, text_y, font_size, self.foreground_color);
        }

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
