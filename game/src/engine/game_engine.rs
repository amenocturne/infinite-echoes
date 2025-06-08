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
use macroquad::time::get_time;
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
use crate::render::widgets::error_popup_widget::ErrorPopupWidget;
use crate::render::widgets::piece_library_widget::PieceLibraryWidget;
use crate::render::widgets::settings_widget::SettingsWidget;
use crate::render::Render;
use crate::render::RenderCtx;
use crate::render::{drag_manager::DragManager, draggable_card_buffer::DraggableCardBuffer};

use super::audio_engine::AudioEngine;
use super::errors::GameResult;
use super::game_config::GameConfig;
use super::game_settings::GameSettings;
use super::game_state::GameEvent;
use super::game_state::GameState;
use super::scheduler::Scheduler;
use super::ton_wallet::{PieceData, TonWallet};

pub struct GameEngine {
    state: RefCell<GameState>,
    render_ctx: RenderCtx,
    config: GameConfig,
    audio_engine: RefCell<AudioEngine>,
    drag_manager: DragManager,
    audio_scheduler: Scheduler,
    audio_graph_widget: AudioGraphWidget,
    cards_row_widget: CardsRowWidget,
    debug_hud: Option<RefCell<DebugHud>>,
    settings_widget: SettingsWidget,
    piece_library_widget: PieceLibraryWidget,
    error_popup_widget: ErrorPopupWidget,
    ton_wallet: RefCell<TonWallet>,
}

impl GameEngine {
    pub fn new(render_ctx: RenderCtx, config: GameConfig) -> GameResult<Self> {
        let settings = GameSettings::default();
        let state = GameState::new(config.initial_deck.clone());

        let screen_w = render_ctx.screen_size.x;
        let screen_h = render_ctx.screen_size.y;
        let card_height_abs = config.card_height * screen_h;
        let card_width_abs = card_height_abs * config.card_aspect_ratio;
        let card_size = vec2(card_width_abs / screen_w, card_height_abs / screen_h);

        let audio_graph_widget = AudioGraphWidget::new(
            config.graph_widget.location,
            config.graph_widget.size,
            card_size,
            config.card_colors.clone(),
        );
        let cards_row_widget = CardsRowWidget::new(
            config.cards_widget.location,
            config.cards_widget.size,
            card_size,
            state.card_deck.clone(),
            config.card_colors.clone(),
        );
        let debug_hud = config
            .debug_hud
            .clone()
            .map(|ref h| RefCell::new(DebugHud::new(h.buffer_size)));

        let audio_engine = AudioEngine::new()?;
        let settings_widget = SettingsWidget::from_settings(settings);
        let piece_library_widget = PieceLibraryWidget::new();
        let error_popup_widget = ErrorPopupWidget::new();

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
            piece_library_widget,
            error_popup_widget,
            ton_wallet: RefCell::new(TonWallet::new()),
        })
    }

    pub async fn update(&mut self) -> GameResult<()> {
        self.update_state();
        self.handle_input().await?;
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

        let wallet = self.ton_wallet.borrow();
        let contract_info = wallet.contract_info();

        let mut pieces_with_metadata: Vec<(&String, &PieceData)> =
            contract_info.piece_data_structs.iter().collect();
        pieces_with_metadata.sort_by(|a, b| b.1.created_at.cmp(&a.1.created_at));

        let total_pieces_to_fetch = contract_info.piece_addresses.len();
        let fetched_pieces_count = contract_info.piece_data.len();
        let is_loading = total_pieces_to_fetch > fetched_pieces_count;

        self.piece_library_widget.render(
            render_ctx,
            &pieces_with_metadata,
            &contract_info.piece_remix_data,
            is_loading,
        )?;

        self.error_popup_widget.render(render_ctx)?;

        Ok(())
    }

    fn update_state(&mut self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        self.render_ctx.screen_size = vec2(screen_w, screen_h);

        let card_height_abs = self.config.card_height * screen_h;
        let card_width_abs = card_height_abs * self.config.card_aspect_ratio;
        let card_size = vec2(card_width_abs / screen_w, card_height_abs / screen_h);

        self.audio_graph_widget.update_card_size(card_size);
        self.cards_row_widget.update_card_size(card_size);
        self.drag_manager.update_dragged_card_size(card_size);

        let buffer_refs: Vec<&dyn DraggableCardBuffer> =
            vec![&self.cards_row_widget, &self.audio_graph_widget];
        self.drag_manager.snap(&buffer_refs);

        self.ton_wallet.borrow_mut().update();

        let mut settings = self.settings_widget.settings.borrow_mut();
        let ton_wallet = self.ton_wallet.borrow();

        settings.vault_address = ton_wallet.user_vault_address().map(|s| s.to_string());
        settings.wallet_address = ton_wallet.user_address().map(|s| s.to_string());
        settings.registry_address = ton_wallet.registry_address().map(|s| s.to_string());
        settings.is_connected = ton_wallet.is_connected();

        let vol = settings.volume;
        self.audio_engine.borrow().set_volume(vol);
    }

    async fn handle_input(&mut self) -> GameResult<()> {
        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_pos = mouse_pos / self.render_ctx.screen_size;

        if self.error_popup_widget.is_visible() {
            if is_key_pressed(KeyCode::Escape) {
                self.error_popup_widget.hide();
            }
            return Ok(());
        }

        if is_key_pressed(KeyCode::Escape) {
            if self.settings_widget.is_visible() || self.piece_library_widget.is_visible() {
                self.settings_widget.hide();
                self.piece_library_widget.hide();
            } else {
                if !self.settings_widget.is_visible() {
                    self.piece_library_widget.hide();
                }
                self.settings_widget.toggle();
            }
        }

        if is_key_pressed(KeyCode::Tab) {
            // If we are opening the library window, close the settings window.
            if !self.piece_library_widget.is_visible() {
                self.settings_widget.hide();
            }
            self.piece_library_widget.toggle();
        }

        if let Some(address) = self.piece_library_widget.handle_load_selection() {
            self.stop_audio_graph()?;
            let wallet = self.ton_wallet.borrow();
            if let Some(cards) = wallet.get_piece_cards(&address) {
                // Load the new cards into the audio graph widget
                self.audio_graph_widget.set_cards(cards.clone());

                // Update game state to track the remix source
                self.state.borrow_mut().remixed_from_address = Some(address);

                // Schedule a graph update and hide the library
                self.audio_scheduler.schedule(GameEvent::UpdateGraph, None);
                self.piece_library_widget.hide();
            }
        }

        if self.settings_widget.handle_new_piece() {
            self.stop_audio_graph()?;

            // Reset widgets
            self.audio_graph_widget.set_cards(vec![]);
            self.cards_row_widget
                .set_cards(self.config.initial_deck.clone());

            // Reset game state
            let mut state = self.state.borrow_mut();
            state.current_graph = None;
            state.playing_graph = None;
            state.playing_cards = None;
            state.remixed_from_address = None;

            self.audio_scheduler.schedule(GameEvent::UpdateGraph, None);
        }

        let mut buffers: Vec<&mut dyn DraggableCardBuffer> =
            vec![&mut self.cards_row_widget, &mut self.audio_graph_widget];

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

        if self.settings_widget.handle_create_piece() {
            let state = self.state.borrow();
            if state.current_graph.is_none() {
                self.error_popup_widget.show(
                    "Invalid audio graph. A valid graph needs at least one note generator and one oscillator.".to_string(),
                );
            } else {
                let cards_to_save = self
                    .audio_graph_widget
                    .cards()
                    .iter()
                    .map(|card| card.borrow().card_type())
                    .collect::<Vec<_>>();

                let piece_name = self.settings_widget.settings.borrow().piece_name.clone();

                let piece_metadata = PieceData {
                    version: 1,
                    name: if piece_name.is_empty() {
                        "Untitled Piece".to_string()
                    } else {
                        piece_name
                    },
                    created_at: get_time() as u64,
                    bpm: self.config.bpm,
                    cards: cards_to_save,
                };

                let piece_data_str = TonWallet::serialize_piece_data(&piece_metadata);
                let remixed_from = state.remixed_from_address.as_deref();
                self.ton_wallet
                    .borrow_mut()
                    .set_pending_piece_data(&piece_data_str, remixed_from);
            }
        }

        if is_key_pressed(KeyCode::A) {
            info!("{:?}", self.ton_wallet.borrow().contract_info());
        }

        Ok(())
    }

    fn stop_audio_graph(&self) -> GameResult<()> {
        self.audio_scheduler.clear();
        self.audio_engine.borrow_mut().stop_all()?;
        Ok(())
    }

    fn process_events(&self) -> GameResult<()> {
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

                    // Get the current cards from the audio graph widget
                    let current_cards = self
                        .audio_graph_widget
                        .cards()
                        .iter()
                        .map(|card| card.borrow().card_type())
                        .collect();

                    let mut state = self.state.borrow_mut();
                    state.playing_graph = Some(audio_graph.clone());
                    state.playing_cards = Some(current_cards);

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
