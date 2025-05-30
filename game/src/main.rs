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
use render::drag_manager::DragManager; // Import the DragManager
use render::draggable_card_buffer::DraggableCardBuffer;
use render::widgets::audio_graph_widget::AudioGraphWidget;
use render::widgets::card_widget::CardType;
use render::widgets::cards_row_widget::CardsRowWidget;
use render::Render;
use render::RenderCtx;
use std::cell::RefCell;

use macroquad::prelude::*;

fn process_event(game_state: &mut GameState, event: &GameEvent) {
    match event {
        GameEvent::StartDrag(_) => {}
        GameEvent::StopDrag => {}
        GameEvent::Drag(_) => {}
        GameEvent::InterpretGraph => {
            if let Some(audio_graph) = &game_state.audio_graph {
                game_state
                    .audio_engine
                    .interpret_graph(audio_graph)
                    .unwrap();
            }
        }
    }
}

fn handle_error(e: GameError) {
    debug!("{:?}", e)
}

async fn run() -> GameResult<()> {
    let audio_engine = AudioEngine::new()?;

    let game_state = RefCell::new(GameState::new(audio_engine));

    let mut scheduler = Scheduler::new();
    let debug_hud = DebugHud::new(100);

    let render_ctx = RenderCtx::new(vec2(screen_width(), screen_height())).await?;

    let mut drag_manager = DragManager::new();

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

    let mut audio_graph_widget = AudioGraphWidget::new(vec2(0.5, 0.5), vec2(0.9, 0.13), card_size);

    loop {
        clear_background(BLACK);
        let screen = vec2(screen_width(), screen_height());
        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_pos = mouse_pos / screen;

        // ---------------------- Input Handling ------------------------------
        {
            let mut buffers: Vec<&mut dyn DraggableCardBuffer> =
                vec![&mut cards_row, &mut audio_graph_widget];

            // Handle mouse input with DragManager - THIS IS THE KEY PART
            if is_mouse_button_pressed(MouseButton::Left) {
                drag_manager.handle_mouse_press(mouse_pos, &mut buffers);
            }

            if is_mouse_button_down(MouseButton::Left) {
                drag_manager.handle_mouse_drag(mouse_pos);
            }

            if is_mouse_button_released(MouseButton::Left) {
                drag_manager.handle_mouse_release(&mut buffers);
            }
        }

        // -------------------- Updating State --------------------------------
        {
            let buffer_refs: Vec<&dyn DraggableCardBuffer> = vec![&cards_row, &audio_graph_widget];
            drag_manager.snap(&buffer_refs);
        }

        // -------------------- Processing Events -----------------------------
        {
            scheduler.process_events(&mut |e| {
                let mut state = game_state.try_borrow_mut().unwrap();
                process_event(&mut state, &e)
            });
        }
        // ---------------------- Rendering -----------------------------------
        {
            cards_row.render(&render_ctx)?;
            audio_graph_widget.render(&render_ctx)?;
            drag_manager.render(&render_ctx)?;
            debug_hud.render(&render_ctx)?;
        }

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
