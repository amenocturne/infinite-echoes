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
use nodes::note_generator::MusicTime;
use nodes::note_generator::Note;
use nodes::note_generator::NoteDuration;
use nodes::note_generator::NoteEvent;
use nodes::note_generator::NoteGenerator;
use nodes::note_generator::NoteName;
use nodes::oscillator::Oscillator;
use nodes::oscillator::WaveShape;
use render::layout::Layout;
use render::widgets::audio_graph_widget::AudioGraphWidget;
use render::widgets::cards_row_widget::CardsRowWidget;
use render::widgets::grid::GridWidget;
use render::Render;
use render::RenderCtx;
use std::cell::RefCell;

use macroquad::prelude::*;

fn process_event(game_state: &mut GameState, event: &GameEvent) {
    match event {
        // Audio Node events
        GameEvent::AudioNodeStartDrag(_) => {}
        GameEvent::AudioNodeStopDrag => {}
        GameEvent::AudioNodeDrag(_) => {}
        GameEvent::AudioNodeAddEffect() => {
            info!("Added effect");
            game_state
                .audio_graph
                .audio_effects
                .push(AudioEffect::new());
        }
        GameEvent::AudioNodeDeleteAudioEffect(_) => {
            // game_state.audio_graph.delete_howered_audio_effect(cursor);
        }

        // rest
        GameEvent::InterpretGraph => {
            game_state
                .audio_engine
                .interpret_graph(&game_state.audio_graph)
                .unwrap(); // TODO: remove unwrap
        }
        GameEvent::ChangeOscillatorType => {
            let new_shape = match game_state.audio_graph.oscillator.wave_shape {
                WaveShape::Sine => WaveShape::Square,
                WaveShape::Square => WaveShape::Sine,
            };
            game_state.audio_graph.oscillator.wave_shape = new_shape;
        }
    }
}

fn handle_error(e: GameError) {
    debug!("{:?}", e)
}

async fn run() -> GameResult<()> {
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

    let audio_graph = AudioGraph::new(note_generator, oscillator, vec![]);
    let audio_engine = AudioEngine::new()?;

    let position = vec2(0.25, 0.25);
    let size = vec2(0.5, 0.5);

    let layout = Layout::new(GridWidget::new(position, size, 5, 2));
    let game_state = RefCell::new(GameState::new(layout, audio_engine, audio_graph));

    let mut scheduler = Scheduler::new();

    let debug_hud = DebugHud::new(100);

    let render_ctx = RenderCtx::new(vec2(screen_width(), screen_height())).await?;

    // let mut card = Card::new(
    //     vec2(0.25, 0.25),
    //     vec2(1.0 / 10.0 - 0.05, 1.0 / 4.0 - 0.05),
    //     WHITE,
    // );

    let card_size = vec2(0.075, 0.1);
    let card_margin = 0.01;
    let cards_row = CardsRowWidget::new(vec2(0.5, 0.85), vec2(0.9, 0.13), card_size, card_margin);

    loop {
        clear_background(BLACK);

        let screen = vec2(screen_width(), screen_height());
        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_pos_relative = mouse_pos / screen;

        if is_mouse_button_pressed(MouseButton::Left) {
            cards_row.start_dragging(mouse_pos_relative);
        }

        if is_mouse_button_released(MouseButton::Left) {
            cards_row.stop_dragging();
        }

        if is_key_pressed(KeyCode::Space) {
            scheduler.schedule(GameEvent::AudioNodeAddEffect(), None);
        }

        cards_row.update_dragged_position(mouse_pos_relative);
        cards_row.snap(0.2);
        cards_row.render(&render_ctx)?;

        let audio_graph_widget = AudioGraphWidget::new(
            vec2(0.5, 0.5),
            vec2(0.9, 0.13),
            card_size,
            card_margin,
            &game_state.borrow().audio_graph,
        );
        audio_graph_widget.render(&render_ctx)?;

        // let cursor = mouse_position().into();
        // if is_mouse_button_pressed(MouseButton::Left) {
        //     if game_state.borrow().audio_graph.is_on_some_node(&cursor) {
        //         scheduler.schedule(GameEvent::AudioNodeStartDrag(cursor), None);
        //     } else {
        //         scheduler.schedule(GameEvent::AudioNodeAddEffect(cursor), None);
        //     }
        // }
        //
        // if is_mouse_button_down(MouseButton::Left) {
        //     scheduler.schedule(GameEvent::AudioNodeDrag(cursor), None);
        // }
        // if is_mouse_button_released(MouseButton::Left) {
        //     scheduler.schedule(GameEvent::AudioNodeStopDrag, None);
        // }
        //
        // if is_mouse_button_pressed(MouseButton::Right) {
        //     if game_state.borrow().audio_graph.is_on_some_node(&cursor) {
        //         scheduler.schedule(GameEvent::AudioNodeDeleteAudioEffect(cursor), None);
        //     }
        // }
        //
        // if is_key_pressed(KeyCode::Space) {
        //     scheduler.schedule(GameEvent::InterpretGraph, None);
        // }
        //
        // if is_key_pressed(KeyCode::A) {
        //     scheduler.schedule(GameEvent::ChangeOscillatorType, None);
        // }

        // // Event processing
        scheduler.process_events(&mut |e| {
            let mut state = game_state.try_borrow_mut().unwrap();
            process_event(&mut state, &e)
        });

        // game_state.borrow().render(&render_ctx)?;
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
