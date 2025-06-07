use crate::engine::errors::GameResult;
use crate::nodes::audio_graph::AudioGraph;
use crate::render::widgets::card_widget::CardType;
use crate::render::Render;
use crate::render::RenderCtx;

pub struct GameState {
    pub current_graph: Option<AudioGraph>,
    pub playing_graph: Option<AudioGraph>,
    pub card_deck: Vec<CardType>,
    pub playing_cards: Option<Vec<CardType>>,
    pub remixed_from_address: Option<String>,
}

impl GameState {
    pub fn new(initial_deck: Vec<CardType>) -> GameState {
        GameState {
            current_graph: None,
            playing_graph: None,
            card_deck: initial_deck,
            playing_cards: None,
            remixed_from_address: None,
        }
    }
}

impl Render for GameState {
    fn render(&self, _render_ctx: &RenderCtx) -> GameResult<()> {
        Ok(())
    }
}

pub enum GameEvent {
    InterpretGraph,
    StopAudioGraph,
    UpdateGraph,
}
