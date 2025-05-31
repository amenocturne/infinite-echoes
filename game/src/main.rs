mod core;
mod debug;
mod engine;
mod nodes;
mod render;

use engine::audio_engine::AudioEngine;
use engine::errors::GameError;
use engine::errors::GameResult;
use engine::game_config::AudioConfig;
use engine::game_config::CardsRowWidgetConfig;
use engine::game_config::DebugHudConfig;
use engine::game_config::GameConfig;
use engine::game_config::GraphWidgetConfig;
use engine::game_engine::GameEngine;
use render::widgets::card_widget::CardType;
use render::RenderCtx;

use macroquad::prelude::*;

fn handle_error(e: GameError) {
    debug!("{:?}", e)
}

async fn run() -> GameResult<()> {
    let game_config = GameConfig {
        card_size: vec2(0.075, 0.1),
        bpm: 120,
        look_ahead_secs: 0.2,
        initial_deck: vec![
            CardType::NoteGenerator,
            CardType::NoteGenerator,
            CardType::SineOscillator,
            CardType::SquareOscillator,
            CardType::Distortion,
            CardType::LowPassFilter,
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
    };

    let audio_engine = AudioEngine::new()?;
    let render_ctx = RenderCtx::new(vec2(screen_width(), screen_height())).await?;
    let mut game_engine = GameEngine::new(audio_engine, render_ctx, game_config);

    loop {
        game_engine.update()?;
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
