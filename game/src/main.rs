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
use macroquad::ui::hash;
use macroquad::ui::root_ui;
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
use render::card::Card;
use render::hover::Hover;
use render::layout::GridWidget;
use render::layout::Layout;
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

    let mut card = Card::new(
        vec2(0.25, 0.25),
        vec2(1.0 / 10.0 - 0.05, 1.0 / 4.0 - 0.05),
        WHITE,
    );

    loop {
        clear_background(BLACK);

        let screen = vec2(screen_width(), screen_height());
        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_pos_relative = mouse_pos / screen;

        if is_mouse_button_pressed(MouseButton::Left) {
            if card.is_hovered_over(mouse_pos_relative) {
                card.start_dragging();
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            card.stop_dragging();
        }

        card.update_dragged_position(mouse_pos_relative);
        let snapping_points = game_state.borrow().layout.grid.snapping_points();
        for p in snapping_points {
            card.snap(p, vec2(0.05, 0.05));
        }
        card.render(&render_ctx)?;

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
        //
        // // Event processing
        // scheduler.process_events(&mut |e| {
        //     let mut state = game_state.try_borrow_mut().unwrap();
        //     process_event(&mut state, &e)
        // });

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
