use std::cell::Cell;
use std::cell::RefCell;

use macroquad::math::vec2;
use macroquad::math::Vec2;
use macroquad::ui::hash;
use macroquad::ui::root_ui;

use crate::engine::game_settings::GameSettings;
use crate::render::RenderCtx;
use crate::{engine::errors::GameResult, render::Render};

pub struct SettingsWidget {
    pub settings: RefCell<GameSettings>,
    is_visible: Cell<bool>,
    position: Vec2,
    size: Vec2,
}

impl SettingsWidget {
    pub fn from_settings(settings: GameSettings) -> Self {
        Self {
            settings: RefCell::new(settings),
            is_visible: Cell::new(false),
            position: vec2(0.5, 0.5),
            size: vec2(0.5, 0.5),
        }
    }

    pub fn toggle(&self) {
        let prev_value = self.is_visible.get();
        self.is_visible.set(!prev_value);
    }
}

impl Render for SettingsWidget {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        if !self.is_visible.get() {
            return Ok(());
        }
        let center = render_ctx.screen_size * self.position;
        let size = self.size * render_ctx.screen_size;
        let position = center - 0.5 * size;

        root_ui().window(hash!(), position, size, |ui| {
            ui.label(None, "Settings Menu");
            let mut settings = self.settings.borrow_mut();
            ui.slider(hash!(), "Volume", 0.0..1.0, &mut settings.volume);
        });
        Ok(())
    }
}
