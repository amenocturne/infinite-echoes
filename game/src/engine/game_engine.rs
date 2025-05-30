use macroquad::color::BLACK;
use macroquad::input::is_key_pressed;
use macroquad::input::is_mouse_button_down;
use macroquad::input::is_mouse_button_pressed;
use macroquad::input::is_mouse_button_released;
use macroquad::input::mouse_position;
use macroquad::math::vec2;
use macroquad::math::Vec2;
use macroquad::window::clear_background;
use macroquad::window::screen_height;
use macroquad::window::screen_width;
use miniquad::info;
use miniquad::KeyCode;
use miniquad::MouseButton;

use crate::debug::hud::DebugHud;
use crate::nodes::audio_graph::AudioGraph;
use crate::render::widgets::audio_graph_widget::AudioGraphWidget;
use crate::render::widgets::cards_row_widget::CardsRowWidget;
use crate::render::Render;
use crate::render::RenderCtx;
use crate::render::{drag_manager::DragManager, draggable_card_buffer::DraggableCardBuffer};

use super::errors::GameResult;
use super::game_state::GameEvent;
use super::{
    audio_engine::AudioEngine, game_config::GameConfig, game_state::GameState, scheduler::Scheduler,
};

pub struct GameEngine {
    state: GameState,
    render_ctx: RenderCtx,
    // engines
    audio_engine: AudioEngine,
    drag_manager: DragManager,
    pub scheduler: Scheduler,
    // TODO: Maybe put them into separate enum for different game states
    audio_graph_widget: AudioGraphWidget,
    cards_row_widget: CardsRowWidget,
    debug_hud: Option<DebugHud>,
}

impl GameEngine {
    pub fn new(audio_engine: AudioEngine, render_ctx: RenderCtx, game_config: GameConfig) -> Self {
        let state = GameState::new(game_config.initial_deck);
        let audio_graph_widget = AudioGraphWidget::new(
            game_config.graph_widget.location,
            game_config.graph_widget.size,
            game_config.card_size,
        );
        let cards_row_widget = CardsRowWidget::new(
            game_config.cards_widget.location,
            game_config.cards_widget.size,
            game_config.card_size,
            state.card_deck.clone(),
        );
        let debug_hud = game_config.debug_hud.map(|h| DebugHud::new(h.buffer_size));
        Self {
            state,
            render_ctx,
            audio_engine,
            drag_manager: DragManager::new(),
            scheduler: Scheduler::new(),
            audio_graph_widget,
            cards_row_widget,
            debug_hud,
        }
    }

    pub fn update(&mut self) -> GameResult<()> {
        self.handle_input();
        self.update_state();
        self.process_events()?;
        self.render()?;
        Ok(())
    }

    fn render(&self) -> GameResult<()> {
        let render_ctx = &self.render_ctx;

        clear_background(BLACK);
        self.cards_row_widget.render(render_ctx)?;
        self.audio_graph_widget.render(render_ctx)?;
        self.drag_manager.render(render_ctx)?;

        if let Some(debug_hud) = &self.debug_hud {
            debug_hud.render(render_ctx)?;
        }

        Ok(())
    }

    fn update_state(&mut self) {
        // Render Ctx
        let screen = vec2(screen_width(), screen_height());
        self.render_ctx.screen_size = screen;

        // Drag Manager
        let buffer_refs: Vec<&dyn DraggableCardBuffer> =
            vec![&self.cards_row_widget, &self.audio_graph_widget];
        self.drag_manager.snap(&buffer_refs);
    }

    fn handle_input(&mut self) {
        let mut buffers: Vec<&mut dyn DraggableCardBuffer> =
            vec![&mut self.cards_row_widget, &mut self.audio_graph_widget];

        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_pos = mouse_pos / self.render_ctx.screen_size;

        // Handle mouse input with DragManager - THIS IS THE KEY PART
        if is_mouse_button_pressed(MouseButton::Left) {
            self.drag_manager
                .handle_mouse_press(mouse_pos, &mut buffers);
        }

        if is_mouse_button_down(MouseButton::Left) {
            self.drag_manager.handle_mouse_drag(mouse_pos);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.drag_manager.handle_mouse_release(&mut buffers);
        }

        if is_key_pressed(KeyCode::Space) {
            self.scheduler.schedule(GameEvent::InterpretGraph, None);
        }
    }

    pub fn interpret_graph(&self) -> GameResult<()> {
        // Get cards from the audio graph widget
        let card_types = self
            .audio_graph_widget
            .cards()
            .iter()
            .map(|card| card.borrow().card_type()) // Convert AudioNodeType to CardType
            .collect();

        if let Some(audio_graph) = AudioGraph::from_cards(card_types) {
            self.audio_engine.interpret_graph(&audio_graph)?;
            self.state.audio_graph.set(Some(audio_graph));
        }

        Ok(())
    }

    fn process_events(&mut self) -> GameResult<()> {
        self.scheduler.process_events(&mut |event| {
            if let Err(e) = self.process_event(event) {
                // Log error but don't crash the game
                println!("Error processing event: {:?}", e);
            }
        });
        Ok(())
    }

    fn process_event(&self, event: GameEvent) -> GameResult<()> {
        match event {
            GameEvent::InterpretGraph => self.interpret_graph(),
        }
    }
}
