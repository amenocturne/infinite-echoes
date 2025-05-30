use macroquad::math::Vec2;

use crate::render::widgets::card_widget::CardType;

pub struct GameConfig {
    pub card_size: Vec2,
    pub initial_deck: Vec<CardType>, // TODO: maybe should be moved to level config
    pub graph_widget: GraphWidgetConfig,
    pub cards_widget: CardsRowWidgetConfig,
    pub debug_hud: Option<DebugHudConfig>
}

pub struct GraphWidgetConfig {
    pub location: Vec2,
    pub size: Vec2,
}

pub struct CardsRowWidgetConfig {
    pub location: Vec2,
    pub size: Vec2,
}

pub struct DebugHudConfig{
    pub buffer_size: usize
}
