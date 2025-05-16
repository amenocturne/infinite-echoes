mod engine;
mod errors;
mod game_state;
mod nodes;
mod render;

use errors::GameError;
use errors::GameResult;
use game_state::GameEvent;
use game_state::GameState;
use nodes::note_generator::Note;
use nodes::note_generator::NoteName;
use nodes::note_generator::NotePosition;
use nodes::oscillator::Oscillator;
use nodes::oscillator::OscillatorState;
use render::rectangle::Rectangle;
use render::Render;
use render::RenderAudio;
use std::collections::VecDeque;

use macroquad::prelude::*;

use web_sys::{AudioContext, OscillatorType};

fn process_event(game_state: &mut GameState, event: &GameEvent) {
    match event {
        GameEvent::OscillatorStart => game_state.oscillator.state = OscillatorState::On,
        GameEvent::OscillatorStop => game_state.oscillator.state = OscillatorState::Off,
        GameEvent::OscillatorDrag { mouse_pos } => {
            game_state.oscillator.rectangle.position =
                *mouse_pos - (0.5 * game_state.oscillator.rectangle.size)
        }
        GameEvent::OscillatorSetFrequency { frequency } => {
            game_state.oscillator.set_frequency(*frequency);
        }
        GameEvent::DraggingStart => game_state.is_dragging = true,
        GameEvent::DraggingStop => game_state.is_dragging = false,
    }
}

fn handle_error(e: GameError) {
    debug!("{:?}", e)
}

async fn run() -> GameResult<()> {
    let audio_context =
        AudioContext::new().map_err(GameError::js("Could not create audio context"))?;
    let osc_rectangle = Rectangle::new(vec2(0.0, 0.0), vec2(50.0, 50.0), RED);

    let note = Note::new(3, NoteName::C, NotePosition::new(1.0, 1.0));

    let oscillator = Oscillator::new(
        OscillatorState::Off,
        note.to_frequancy(),
        OscillatorType::Sine,
        osc_rectangle,
    );

    let mut game_state = GameState::new(audio_context, oscillator)?;
    let mut event_queue: VecDeque<GameEvent> = VecDeque::new();

    fn chord(start: Note, shifts: Vec<i32>) -> Vec<Note> {
        shifts.iter().map(|s| start.clone().shift(*s)).collect()
    }
    fn major_chord(start: Note) -> Vec<Note> {
        chord(start, vec![0, 4, 7])
    }
    fn minor_chord(start: Note) -> Vec<Note> {
        chord(start, vec![0, 3, 7])
    }
    let notes = [
        major_chord(Note::new(3, NoteName::C, NotePosition::new(0.0, 1.0))),
        minor_chord(Note::new(3, NoteName::D, NotePosition::new(1.0, 1.0))),
        minor_chord(Note::new(3, NoteName::E, NotePosition::new(2.0, 1.0))),
        major_chord(Note::new(3, NoteName::C, NotePosition::new(3.0, 1.0))),
    ]
    .concat();

    loop {
        clear_background(BLACK);
        let mouse_pos: Vec2 = mouse_position().into();
        // let width = screen_width();

        // Input
        {
            let mut emit = |event: GameEvent| event_queue.push_back(event);

            if is_mouse_button_pressed(MouseButton::Left) {
                // note_selector = (note_selector + 1) % notes.len();
                // info!("{}", note_selector);
                // emit(GameEvent::OscillatorSetFrequency {
                //     frequency: unsafe { notes.get_unchecked(note_selector).to_frequancy() },
                // });
                //
                // emit(GameEvent::OscillatorStart);
                // emit(GameEvent::DraggingStart);
                game_state.audio_engine.play_notes(notes.as_slice())?;
            }

            if is_mouse_button_released(MouseButton::Left) {
                emit(GameEvent::OscillatorStop);
                emit(GameEvent::DraggingStop);
            }

            // Additional events based on the state
            if game_state.is_dragging {
                emit(GameEvent::OscillatorDrag { mouse_pos });
            }
        }

        // Event processing
        while let Some(event) = event_queue.pop_front() {
            process_event(&mut game_state, &event);
        }

        //Rendering
        game_state.render_audio(&game_state.audio_context)?;
        game_state.render()?;
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
