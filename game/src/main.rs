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
use render::drag_manager::DragManager; // Import the DragManager
use render::draggable_card_buffer::DraggableCardBuffer;
use render::layout::Layout;
use render::widgets::audio_graph_widget::AudioGraphWidget;
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
        GameEvent::InterpretGraph => {
            if game_state.audio_graph.is_valid() {
                game_state
                    .audio_engine
                    .interpret_graph(&game_state.audio_graph)
                    .unwrap();
            }
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

    // Initialize DragManager
    let mut drag_manager = DragManager::new();

    let card_size = vec2(0.075, 0.1);

    // Create card buffers
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

    loop {
        clear_background(BLACK);

        let screen = vec2(screen_width(), screen_height());
        let mouse_pos_absolute: Vec2 = mouse_position().into();
        let mouse_pos = mouse_pos_absolute / screen;

        // Create array of mutable references to buffers for DragManager
        let mut buffers: Vec<&mut dyn DraggableCardBuffer> =
            vec![&mut cards_row, &mut audio_graph_widget];

        // Handle mouse input with DragManager - THIS IS THE KEY PART
        if is_mouse_button_pressed(MouseButton::Left) {
            drag_manager.handle_mouse_press(mouse_pos, &mut buffers);
        }

        if is_mouse_button_down(MouseButton::Left) {
            info!("{:?}", drag_manager);
            drag_manager.handle_mouse_drag(mouse_pos);
        }

        if is_mouse_button_released(MouseButton::Left) {
            drag_manager.handle_mouse_release(&mut buffers);
        }

        // Handle keyboard shortcuts for testing
        if is_key_pressed(KeyCode::Escape) {
            drag_manager.abort_all_dragging(&mut buffers);
        }

        // Update buffers (snapping, organizing, etc.)
        {
            let buffer_refs: Vec<&dyn DraggableCardBuffer> = vec![&cards_row, &audio_graph_widget];
            drag_manager.snap(&buffer_refs);
        }

        // Update audio graph widget to reflect current game state
        // audio_graph_widget.update_audio_graph(&game_state.borrow().audio_graph);

        // Render everything
        cards_row.render(&render_ctx)?;
        audio_graph_widget.render(&render_ctx)?;

        // Render the dragged card (handled by DragManager)
        drag_manager.render(&render_ctx)?;

        // Event processing
        scheduler.process_events(&mut |e| {
            let mut state = game_state.try_borrow_mut().unwrap();
            process_event(&mut state, &e)
        });

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
