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
use nodes::audio_effect::AudioEffectType;
use nodes::audio_graph::AudioGraph;
use nodes::note_generator::MusicTime;
use nodes::note_generator::Note;
use nodes::note_generator::NoteDuration;
use nodes::note_generator::NoteEvent;
use nodes::note_generator::NoteGenerator;
use nodes::note_generator::NoteName;
use nodes::oscillator::Oscillator;
use nodes::oscillator::WaveShape;
use nodes::AudioNode;
use render::layout::Layout;
use render::widgets::audio_graph_widget::AudioGraphWidget;
use render::widgets::card_widget::Card;
use render::widgets::card_widget::CardType;
use render::widgets::cards_row_widget::CardsRowWidget;
use render::widgets::grid_widget::GridWidget;
use render::Render;
use render::RenderCtx;
use std::cell::RefCell;
use std::usize;

use macroquad::prelude::*;

fn process_event(game_state: &mut GameState, event: &GameEvent) {
    match event {
        // Audio Node events
        GameEvent::StartDrag(_) => {}
        GameEvent::StopDrag => {}
        GameEvent::Drag(_) => {}
        GameEvent::AddAudioEffect => {
            game_state.audio_graph.add(
                AudioNode::AudioEffect(AudioEffect::new(AudioEffectType::Filter)),
                usize::MAX,
            );
        }
        GameEvent::DeleteAudioEffect => {
            game_state.audio_graph.remove(usize::MAX);
        }

        GameEvent::AddNoteGenerator => {
            game_state.audio_graph.add(
                AudioNode::NoteGenerator(NoteGenerator::new(NoteDuration::Whole.into(), vec![])),
                usize::MAX,
            );
        }
        GameEvent::DeleteNoteGenerator => {
            game_state.audio_graph.remove(usize::MAX);
        }

        // rest
        GameEvent::InterpretGraph => {
            if game_state.audio_graph.is_valid() {
                game_state
                    .audio_engine
                    .interpret_graph(&game_state.audio_graph)
                    .unwrap(); // TODO: remove unwrap
            }
        } // GameEvent::ChangeOscillatorType => {
          //     let new_shape = match game_state.audio_graph.oscillator.wave_shape {
          //         WaveShape::Sine => WaveShape::Square,
          //         WaveShape::Square => WaveShape::Sine,
          //     };
          //     game_state.audio_graph.oscillator.wave_shape = new_shape;
          // }
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

    let audio_graph = AudioGraph::new(
        vec![note_generator.clone(), note_generator.clone()],
        oscillator,
        vec![],
    );
    let audio_engine = AudioEngine::new()?;

    let position = vec2(0.25, 0.25);
    let size = vec2(0.5, 0.5);

    let layout = Layout::new(GridWidget::new(position, size, 5, 2));
    let game_state = RefCell::new(GameState::new(layout, audio_engine, audio_graph));

    let mut scheduler = Scheduler::new();

    let debug_hud = DebugHud::new(100);

    let render_ctx = RenderCtx::new(vec2(screen_width(), screen_height())).await?;

    let card_size = vec2(0.075, 0.1);
    let mut cards_row = CardsRowWidget::new(
        vec2(0.5, 0.85),
        vec2(0.9, 0.13),
        card_size,
        vec![
            CardType::NoteGenerator,
            CardType::SineOscillator,
            CardType::AudioEffect,
            CardType::AudioEffect,
        ],
    );
    let mut audio_graph_widget = AudioGraphWidget::new(
        vec2(0.5, 0.5),
        vec2(0.9, 0.13),
        card_size,
        &game_state.borrow().audio_graph,
    );

    let mut dragged_card = Some(RefCell::new(Card::new(
        vec2(0.5, 0.5),
        card_size,
        RED,
        RED,
        CardType::AudioEffect,
    )));

    loop {
        clear_background(BLACK);

        let screen = vec2(screen_width(), screen_height());
        let mouse_pos_absolute: Vec2 = mouse_position().into();
        let mouse_pos = mouse_pos_absolute / screen;

        if is_mouse_button_pressed(MouseButton::Left) {
            dragged_card
                .as_ref()
                .map(|rc| rc.borrow_mut().start_dragging());
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(card_ref) = &dragged_card {
                card_ref.borrow_mut().update_dragged_position(mouse_pos);
                if cards_row.add_card(card_ref) {
                    if let Some(_) = dragged_card.take() {
                        cards_row.abort_dragging();
                    }
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragged_card
                .as_ref()
                .map(|rc| rc.borrow_mut().stop_dragging());
        }

        dragged_card
            .as_ref()
            .map(|rc| rc.borrow().render(&render_ctx))
            .unwrap_or(Ok(()))?;

        cards_row.update_dragged_position(mouse_pos);
        cards_row.snap(0.2);
        cards_row.render(&render_ctx)?;

        // Event processing
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
