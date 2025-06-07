use std::cell::Cell;
use std::cell::RefCell;

use macroquad::math::vec2;
use macroquad::math::Vec2;
use macroquad::ui::hash;
use macroquad::ui::root_ui;

use crate::engine::game_settings::GameSettings;
use crate::render::RenderCtx;
use crate::{engine::errors::GameResult, render::Render};
use macroquad::prelude::*;
use macroquad::ui::Skin;

pub struct SettingsWidget {
    pub settings: RefCell<GameSettings>,
    is_visible: Cell<bool>,
    position: Vec2,
    size: Vec2,
    create_piece_clicked: Cell<bool>,
}

impl SettingsWidget {
    pub fn from_settings(settings: GameSettings) -> Self {
        Self {
            settings: RefCell::new(settings),
            is_visible: Cell::new(false),
            position: vec2(0.5, 0.5),
            size: vec2(0.5, 0.5),
            create_piece_clicked: Cell::new(false),
        }
    }

    pub fn toggle(&self) {
        let prev_value = self.is_visible.get();
        self.is_visible.set(!prev_value);
    }

    pub fn handle_create_piece(&self) -> bool {
        let prev = self.create_piece_clicked.get();
        self.create_piece_clicked.set(false);
        return prev;
    }
}

impl Render for SettingsWidget {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        if !self.is_visible.get() {
            return Ok(());
        }
        let center = render_ctx.screen_size * self.position;
        let size = self.size * render_ctx.screen_size;
        let top_left = center - 0.5 * size;

        let dark_skin = {
            let label_style = root_ui()
                .style_builder()
                .text_color(WHITE)
                .font_size(30)
                .build();

            let black_image = Image::gen_image_color(1, 1, BLACK);
            let window_style = root_ui()
                .style_builder()
                .background(black_image)
                .background_margin(RectOffset::new(0., 0., 0., 0.))
                .margin(RectOffset::new(0., 0., 0., 0.))
                .build();

            Skin {
                window_style,
                label_style,
                ..root_ui().default_skin()
            }
        };

        let mut settings = self.settings.borrow_mut();

        root_ui().push_skin(&dark_skin);
        root_ui().window(hash!(), top_left, size, |ui| {
            let mut opt_short_label = |name: &'static str, str: Option<String>| {
                ui.label(
                    None,
                    format!(
                        "{}: {}",
                        name,
                        str.map(|addr| make_short(&addr.clone(), 6))
                            .unwrap_or("None".to_string())
                    )
                    .as_str(),
                );
            };

            opt_short_label("Wallet", settings.wallet_address.clone());
            opt_short_label("Registry", settings.registry_address.clone());
            opt_short_label("Vault", settings.vault_address.clone());

            ui.label(None, "");
            ui.label(None, "Settings:");
            ui.slider(hash!(), "Volume", 0.0..1.0, &mut settings.volume);

            if settings.is_connected {
                let button_height = 40.0;
                let button_width = 8.0 * 10.0;
                let button_y = size.y - button_height - 10.0;
                let button_x = (size.x - button_width) * 0.5;
                let button_clicked = ui.button(vec2(button_x, button_y), "Save Piece");
                self.create_piece_clicked.set(button_clicked);
            }
        });
        root_ui().pop_skin();

        let border_thickness = 2.0;
        draw_rectangle_lines(
            top_left.x - border_thickness,
            top_left.y - border_thickness,
            size.x + 2.0 * border_thickness,
            size.y + 2.0 * border_thickness,
            border_thickness,
            WHITE,
        );

        Ok(())
    }
}

fn make_short(string: &str, num_chars: usize) -> String {
    if string.len() <= num_chars {
        string.to_string()
    } else {
        format!(
            "{}...{}",
            string.chars().take(num_chars).collect::<String>(),
            string.chars().rev().take(num_chars).collect::<String>()
        )
    }
}
