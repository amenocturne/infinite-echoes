mod core;
mod debug;
mod engine;
mod nodes;
mod render;

use crate::nodes::audio_effect::FilterType;
use engine::errors::GameError;
use engine::errors::GameResult;
use engine::game_config::AudioConfig;
use engine::game_config::CardColorConfig;
use engine::game_config::CardsRowWidgetConfig;
use engine::game_config::DebugHudConfig;
use engine::game_config::GameConfig;
use engine::game_config::GraphWidgetConfig;
use engine::game_engine::GameEngine;
use nodes::note_effect::ChangeLenType;
use nodes::note_effect::ScaleType;
use nodes::note_generator::NoteName;
use nodes::oscillator::WaveShape;
use render::widgets::card_widget::CardType;
use render::RenderCtx;

use macroquad::prelude::*;

fn handle_error(e: GameError) {
    debug!("{:?}", e)
}

async fn run() -> GameResult<()> {
    let game_config = GameConfig {
        card_height: 0.1,
        card_aspect_ratio: 0.75,
        bpm: 120,
        look_ahead_secs: 0.2,
        initial_deck: vec![
            CardType::NoteGenerator(NoteName::DSharp),
            CardType::NoteGenerator(NoteName::DSharp),
            CardType::NoteGenerator(NoteName::DSharp),
            CardType::NoteGenerator(NoteName::G),
            CardType::NoteGenerator(NoteName::F),
            CardType::NoteGenerator(NoteName::F),
            CardType::NoteGenerator(NoteName::F),
            CardType::NoteGenerator(NoteName::D),
            CardType::ChangeLen(ChangeLenType::Tripplets),
            CardType::ChangeLen(ChangeLenType::Tripplets),
            CardType::ChangeLen(ChangeLenType::Half),
            CardType::ChangeLen(ChangeLenType::Double),
            CardType::ChordInScale(NoteName::C, ScaleType::Major),
            CardType::ChordInScale(NoteName::A, ScaleType::Minor),
            CardType::Oscillator(WaveShape::Sine),
            CardType::Oscillator(WaveShape::Square),
            CardType::Distortion,
            CardType::Reverb,
            CardType::Filter(FilterType::LowPass),
        ],
        graph_widget: GraphWidgetConfig {
            location: vec2(0.5, 0.5),
            size: vec2(0.9, 0.13),
        },
        cards_widget: CardsRowWidgetConfig {
            location: vec2(0.5, 0.85),
            size: vec2(0.9, 0.13),
        },
        debug_hud: Some(DebugHudConfig { buffer_size: 100 }),
        audio: AudioConfig {
            attack_time: 0.001,
            release_time: 0.002,
            max_schedule_ahead: 120.0,
            output_gain: 0.8,
        },
        card_colors: CardColorConfig {
            note_generator: Color::from_hex(0xF7567C),
            note_effect: Color::from_hex(0xFCBA04),
            oscillator: Color::from_hex(0x99E1D9),
            audio_effect: Color::from_hex(0xC2AED6),
        },
    };

    let render_ctx = RenderCtx::new(vec2(screen_width(), screen_height())).await?;
    let mut game_engine = GameEngine::new(render_ctx, game_config)?;

    loop {
        game_engine.update().await?;
        next_frame().await;
    }
}

#[macroquad::main("Infinite Echoes")]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => handle_error(e),
    }
}
