mod debug;
mod engine;
mod nodes;
mod render;
mod util;

use debug::hud::DebugHud;
use engine::audio_engine::AudioEngine;
use engine::errors::GameError;
use engine::errors::GameResult;
use engine::game_state::GameEvent;
use engine::game_state::GameState;
use engine::scheduler::Scheduler;
use macros::note;
use nodes::audio_graph::AudioGraph;
use nodes::note_generator::MusicTime;
use nodes::note_generator::Note;
use nodes::note_generator::NoteDuration;
use nodes::note_generator::NoteEvent;
use nodes::note_generator::NoteGenerator;
use nodes::note_generator::NoteName;
use nodes::oscillator::Oscillator;
use nodes::oscillator::WaveShape;
use render::Render;
use std::cell::RefCell;

use macroquad::prelude::*;

fn process_event(game_state: &mut GameState, event: &GameEvent) {
    match event {
        GameEvent::InterpretGraph => {
            game_state
                .audio_engine
                .interpret_graph(&game_state.audio_graph);
            info!("InterpretGraph");
        }
    }
}

fn handle_error(e: GameError) {
    debug!("{:?}", e)
}

async fn run() -> GameResult<()> {
    let audio_engine = AudioEngine::new()?;

    let note_generator = NoteGenerator::new(
        NoteDuration::Whole.into(),
        vec![
            NoteEvent::new(note!("C3"), MusicTime::ZERO, NoteDuration::Quarter.into()),
            NoteEvent::new(
                note!("D3"),
                NoteDuration::Quarter.into(),
                NoteDuration::Quarter.into(),
            ),
            NoteEvent::new(
                note!("E3"),
                NoteDuration::Half.into(),
                NoteDuration::Quarter.into(),
            ),
        ],
    );

    let oscillator = Oscillator::new(WaveShape::Sine);
    let audio_graph = AudioGraph::new(note_generator, oscillator);

    let game_state = RefCell::new(GameState::new(audio_engine, audio_graph));

    let mut scheduler = Scheduler::new();

    let debug_hud = DebugHud::new(100);

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            scheduler.schedule(GameEvent::InterpretGraph, None);
            info!("Pressed key")
        }
        // Event processing
        scheduler.process_events(&mut |e| {
            let mut state = game_state.try_borrow_mut().unwrap();
            process_event(&mut state, &e)
        });

        game_state.borrow().render()?;
        debug_hud.render()?;
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
