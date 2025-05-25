mod core;
mod debug;
mod engine;
mod nodes;
mod render;

use debug::hud::DebugHud;
use engine::audio_engine::AudioEngine;
use engine::errors::GameError;
use engine::errors::GameResult;
use engine::game_state::GameEvent;
use engine::game_state::GameState;
use engine::scheduler::Scheduler;
use macros::note;
use nodes::audio_effect::AudioEffect;
use nodes::audio_graph::AudioGraph;
use nodes::audio_node::DisplayedAudioNode;
use nodes::note_generator::MusicTime;
use nodes::note_generator::Note;
use nodes::note_generator::NoteDuration;
use nodes::note_generator::NoteEvent;
use nodes::note_generator::NoteGenerator;
use nodes::note_generator::NoteName;
use nodes::oscillator::Oscillator;
use nodes::oscillator::WaveShape;
use render::shapes::Shape;
use render::Render;
use render::RenderCtx;
use std::cell::RefCell;

use macroquad::prelude::*;

fn process_event(game_state: &mut GameState, event: &GameEvent) {
    match event {
        // Audio Node events
        GameEvent::AudioNodeStartDrag(cursor) => {
            for n in game_state.audio_graph.all_nodes() {
                n.maybe_start_dragging(cursor);
            }
        }
        GameEvent::AudioNodeStopDrag => {
            for n in game_state.audio_graph.all_nodes() {
                n.stop_dragging();
            }
        }
        GameEvent::AudioNodeDrag(cursor) => {
            for n in game_state.audio_graph.all_nodes() {
                n.update_dragged_position(cursor);
            }
        }
        GameEvent::AudioNodeAddEffect(cursor) => {
            let audio_effect_displayed =
                DisplayedAudioNode::new(*cursor, vec2(50.0, 50.0), WHITE, BLUE, Shape::Blank); // TODO: remove
            game_state
                .audio_graph
                .audio_effects
                .push(AudioEffect::new(audio_effect_displayed));
        }
        GameEvent::AudioNodeDeleteAudioEffect(cursor) => {
            game_state.audio_graph.delete_howered_audio_effect(cursor);
        }

        // rest
        GameEvent::InterpretGraph => {
            game_state
                .audio_engine
                .interpret_graph(&game_state.audio_graph);
        }
    }
}

fn handle_error(e: GameError) {
    debug!("{:?}", e)
}

async fn run() -> GameResult<()> {
    let note_generator_displayed = DisplayedAudioNode::new(
        vec2(100.0, 100.0),
        vec2(50.0, 50.0),
        WHITE,
        GRAY,
        Shape::Piano,
    ); // TODO: remove
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
        note_generator_displayed,
    );

    let oscillator_displayed = DisplayedAudioNode::new(
        vec2(200.0, 200.0),
        vec2(50.0, 50.0),
        WHITE,
        GREEN,
        Shape::SineWave,
    ); // TODO: remove
    let oscillator = Oscillator::new(WaveShape::Sine, oscillator_displayed);

    let audio_graph = AudioGraph::new(note_generator, oscillator, vec![]);
    let audio_engine = AudioEngine::new()?;
    let game_state = RefCell::new(GameState::new(audio_engine, audio_graph));

    let mut scheduler = Scheduler::new();

    let debug_hud = DebugHud::new(100);

    let render_ctx = RenderCtx::new().await?;

    loop {
        clear_background(BLACK);
        let cursor = mouse_position().into();

        if is_mouse_button_pressed(MouseButton::Left) {
            if game_state.borrow().audio_graph.is_on_some_node(&cursor) {
                scheduler.schedule(GameEvent::AudioNodeStartDrag(cursor), None);
            } else {
                scheduler.schedule(GameEvent::AudioNodeAddEffect(cursor), None);
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            scheduler.schedule(GameEvent::AudioNodeDrag(cursor), None);
        }
        if is_mouse_button_released(MouseButton::Left) {
            scheduler.schedule(GameEvent::AudioNodeStopDrag, None);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            if game_state.borrow().audio_graph.is_on_some_node(&cursor) {
                scheduler.schedule(GameEvent::AudioNodeDeleteAudioEffect(cursor), None);
            }
        }

        // Event processing
        scheduler.process_events(&mut |e| {
            let mut state = game_state.try_borrow_mut().unwrap();
            process_event(&mut state, &e)
        });

        game_state.borrow().render(&render_ctx)?;
        debug_hud.render(&render_ctx)?;
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
