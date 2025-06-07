use std::cell::RefCell;
use std::time::Duration;

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
use miniquad::KeyCode;
use miniquad::MouseButton;

use crate::debug::hud::DebugHud;
use crate::nodes::audio_graph::AudioGraph;
use crate::render::widgets::audio_graph_widget::AudioGraphWidget;
use crate::render::widgets::cards_row_widget::CardsRowWidget;
use crate::render::widgets::settings_widget::SettingsWidget;
use crate::render::Render;
use crate::render::RenderCtx;
use crate::render::{drag_manager::DragManager, draggable_card_buffer::DraggableCardBuffer};

use super::errors::GameResult;
use super::game_settings::GameSettings;
use super::game_state::GameEvent;
use super::ton_wallet::TonWallet;
use super::{
    audio_engine::AudioEngine, game_config::GameConfig, game_state::GameState, scheduler::Scheduler,
};

pub struct GameEngine {
    state: RefCell<GameState>,
    render_ctx: RenderCtx,
    config: GameConfig,
    // engines
    audio_engine: RefCell<AudioEngine>,
    drag_manager: DragManager,
    audio_scheduler: Scheduler,
    // UI
    audio_graph_widget: AudioGraphWidget,
    cards_row_widget: CardsRowWidget,
    debug_hud: Option<RefCell<DebugHud>>,
    settings_widget: SettingsWidget,
    // TON wallet integration
    ton_wallet: RefCell<TonWallet>,
}

impl GameEngine {
    pub fn new(render_ctx: RenderCtx, config: GameConfig) -> GameResult<Self> {
        let settings = GameSettings::default();
        let state = GameState::new(config.initial_deck.clone());
        let audio_graph_widget = AudioGraphWidget::new(
            config.graph_widget.location,
            config.graph_widget.size,
            config.card_size,
        );
        let cards_row_widget = CardsRowWidget::new(
            config.cards_widget.location,
            config.cards_widget.size,
            config.card_size,
            state.card_deck.clone(),
        );
        let debug_hud = config
            .debug_hud
            .clone()
            .map(|ref h| RefCell::new(DebugHud::new(h.buffer_size)));

        let audio_engine = AudioEngine::new()?;
        let settings_widget = SettingsWidget::from_settings(settings);

        Ok(Self {
            state: RefCell::new(state),
            render_ctx,
            config,
            audio_engine: RefCell::new(audio_engine),
            drag_manager: DragManager::new(),
            audio_scheduler: Scheduler::new(),
            audio_graph_widget,
            cards_row_widget,
            debug_hud,
            settings_widget,
            ton_wallet: RefCell::new(TonWallet::new()),
        })
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
            debug_hud.borrow().render(render_ctx)?;
        }

        self.settings_widget.render(render_ctx)?;

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

        // Update TON wallet status
        self.ton_wallet.borrow_mut().update();

        // Update debug HUD with wallet information
        if let Some(debug_hud) = &self.debug_hud {
            let wallet = self.ton_wallet.borrow();
            if let Some(vault_address) = wallet.user_vault_address() {
                debug_hud.borrow_mut().update_vault_addr(vault_address);
            }
        }

        let vol = self.settings_widget.settings.borrow().volume;
        self.audio_engine.borrow().set_volume(vol);
    }

    fn handle_input(&mut self) {
        let mut buffers: Vec<&mut dyn DraggableCardBuffer> =
            vec![&mut self.cards_row_widget, &mut self.audio_graph_widget];

        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_pos = mouse_pos / self.render_ctx.screen_size;

        if is_key_pressed(KeyCode::Escape) {
            self.settings_widget.toggle();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.drag_manager
                .handle_mouse_press(mouse_pos, &mut buffers);
        }

        if is_mouse_button_down(MouseButton::Left) {
            self.drag_manager.handle_mouse_drag(mouse_pos);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.drag_manager.handle_mouse_release(&mut buffers);
            self.audio_scheduler.schedule(GameEvent::UpdateGraph, None)
        }

        let is_playing = self.audio_engine.borrow().is_playing();
        let is_different = self.state.borrow().current_graph != self.state.borrow().playing_graph;
        let should_interpert = !is_playing || is_different;
        if is_key_pressed(KeyCode::Space) && should_interpert {
            self.audio_scheduler
                .schedule(GameEvent::InterpretGraph, None);
        }
        if is_key_pressed(KeyCode::Space) && !should_interpert {
            self.audio_scheduler
                .schedule(GameEvent::StopAudioGraph, None);
        }
    }

    fn stop_audio_graph(&self) -> GameResult<()> {
        self.audio_scheduler.clear(); // TODO: maybe should be removed
        self.audio_engine.borrow_mut().stop_all()?;
        Ok(())
    }

    fn process_events(&mut self) -> GameResult<()> {
        self.audio_scheduler
            .process_events(&mut |event| self.process_event(event));
        Ok(())
    }

    fn process_event(&self, event: GameEvent) -> GameResult<Vec<(GameEvent, Option<Duration>)>> {
        match event {
            GameEvent::InterpretGraph => {
                let maybe_graph = self.state.borrow().current_graph.clone();
                if let Some(audio_graph) = maybe_graph {
                    self.stop_audio_graph()?;
                    self.audio_engine.borrow_mut().interpret_graph(
                        self.config.bpm,
                        &audio_graph,
                        &self.config.audio,
                    )?;

                    // Update playing_graph after we've used audio_graph
                    self.state.borrow_mut().playing_graph = Some(audio_graph.clone());

                    Ok(vec![])
                } else {
                    Ok(vec![(GameEvent::StopAudioGraph, None)])
                }
            }
            GameEvent::StopAudioGraph => {
                self.stop_audio_graph()?;
                Ok(vec![])
            }
            GameEvent::UpdateGraph => {
                let card_types = self
                    .audio_graph_widget
                    .cards()
                    .iter()
                    .map(|card| card.borrow().card_type())
                    .collect();
                self.state.borrow_mut().current_graph = AudioGraph::from_cards(card_types).clone();
                Ok(vec![])
            }
        }
    }
}
