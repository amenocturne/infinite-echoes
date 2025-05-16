mod engine;
mod nodes;
mod render;
mod util;

use engine::audio::AudioEngine;
use engine::errors::GameError;
use engine::errors::GameResult;
use engine::game_state::GameEvent;
use engine::game_state::GameState;
use engine::scheduler::Scheduler;
use nodes::note_generator::Note;
use nodes::note_generator::NoteName;
use nodes::note_generator::NotePosition;
use render::Render;
use std::cell::RefCell;
use std::time::Duration;

use macroquad::prelude::*;

fn process_event(game_state: &mut GameState, event: &GameEvent) {
    match event {
        GameEvent::DisplayRectangle => game_state.is_rectangle_visible = true,
    }
}

fn handle_error(e: GameError) {
    debug!("{:?}", e)
}

async fn run() -> GameResult<()> {
    let audio_engine = AudioEngine::new()?;
    let mut game_state = RefCell::new(GameState::new(audio_engine));

    let mut scheduler = Scheduler::new();

    fn chord(start: Note, shifts: &Vec<i32>) -> Vec<Note> {
        shifts.iter().map(|s| start.clone().shift(*s)).collect()
    }
    let major = vec![0, 4, 7];
    let minor = vec![0, 3, 7];

    let notes = [
        chord(
            Note::new(3, NoteName::C, NotePosition::new(0.0, 1.0)),
            &major,
        ),
        chord(
            Note::new(3, NoteName::D, NotePosition::new(1.0, 1.0)),
            &minor,
        ),
        chord(
            Note::new(3, NoteName::E, NotePosition::new(2.0, 1.0)),
            &minor,
        ),
        chord(
            Note::new(3, NoteName::C, NotePosition::new(3.0, 1.0)),
            &major,
        ),
    ]
    .concat();

    scheduler.schedule(GameEvent::DisplayRectangle, Some(Duration::from_secs(1)));
    loop {
        clear_background(BLACK);
        // Event processing
        scheduler.process_events(&mut |e| {
            let mut state = game_state
                .try_borrow_mut()
                .unwrap();
            process_event(&mut state, &e)
        });

        game_state.borrow().render()?;
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
