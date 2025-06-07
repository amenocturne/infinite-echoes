use macroquad::color::Color;
use macroquad::math::Vec2;

use crate::render::widgets::card_widget::CardType;

#[derive(Clone)]
pub struct GameConfig {
    pub card_height: f32,
    pub card_aspect_ratio: f32, // width / height
    pub bpm: u32,
    pub look_ahead_secs: f64,
    pub initial_deck: Vec<CardType>,
    pub graph_widget: GraphWidgetConfig,
    pub cards_widget: CardsRowWidgetConfig,
    pub debug_hud: Option<DebugHudConfig>,
    pub audio: AudioConfig,
    pub card_colors: CardColorConfig,
}

#[derive(Clone, Debug)]
pub struct CardColorConfig {
    pub note_generator: Color,
    pub note_effect: Color,
    pub oscillator: Color,
    pub audio_effect: Color,
}

#[derive(Clone)]
pub struct GraphWidgetConfig {
    pub location: Vec2,
    pub size: Vec2,
}

#[derive(Clone)]
pub struct CardsRowWidgetConfig {
    pub location: Vec2,
    pub size: Vec2,
}

#[derive(Clone)]
pub struct DebugHudConfig {
    pub buffer_size: usize,
}

#[derive(Clone)]
pub struct AudioConfig {
    pub attack_time: f64,
    pub release_time: f64,
    pub max_schedule_ahead: f64,
    pub output_gain: f32,
}
