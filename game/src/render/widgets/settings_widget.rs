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
    new_piece_clicked: Cell<bool>,
}

impl SettingsWidget {
    pub fn from_settings(settings: GameSettings) -> Self {
        Self {
            settings: RefCell::new(settings),
            is_visible: Cell::new(false),
            position: vec2(0.5, 0.5),
            size: vec2(0.5, 0.5),
            create_piece_clicked: Cell::new(false),
            new_piece_clicked: Cell::new(false),
        }
    }

    pub fn toggle(&self) {
        let prev_value = self.is_visible.get();
        self.is_visible.set(!prev_value);
    }

    pub fn hide(&self) {
        self.is_visible.set(false);
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible.get()
    }

    pub fn handle_create_piece(&self) -> bool {
        let prev = self.create_piece_clicked.get();
        self.create_piece_clicked.set(false);
        return prev;
    }

    pub fn handle_new_piece(&self) -> bool {
        let prev = self.new_piece_clicked.get();
        self.new_piece_clicked.set(false);
        return prev;
    }
}

impl Render for SettingsWidget {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        if !self.is_visible.get() {
            return Ok(());
        }
        let center = self.position * render_ctx.screen_size;
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
        root_ui().window(
            hash!(
                "settings",
                render_ctx.screen_size.x.to_bits(),
                render_ctx.screen_size.y.to_bits()
            ),
            top_left,
            size,
            |ui| {
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

                ui.label(None, "Piece Name:");
                ui.editbox(hash!(), vec2(size.x * 0.8, 30.0), &mut settings.piece_name);

                if settings.is_connected {
                    let button_spacing = 10.0;
                    let init_text = "Init".to_string();
                    let init_size = ui.calc_size(&init_text);

                    let save_text = "Save".to_string();
                    let save_size = ui.calc_size(&save_text);

                    let button_padding_x = 20.0;
                    let new_piece_button_width = init_size.x + button_padding_x;
                    let save_piece_button_width = save_size.x + button_padding_x;
                    let button_height = init_size.y.max(save_size.y) + 10.0;

                    let total_width =
                        new_piece_button_width + button_spacing + save_piece_button_width;

                    let bottom_padding = 20.0;
                    let button_y = size.y - button_height - bottom_padding;
                    let start_x = (size.x - total_width) / 2.0;

                    let new_piece_pos = vec2(start_x, button_y);
                    let save_piece_pos =
                        vec2(start_x + new_piece_button_width + button_spacing, button_y);

                    if ui.button(Some(new_piece_pos), init_text) {
                        self.new_piece_clicked.set(true);
                    }

                    if ui.button(Some(save_piece_pos), save_text) {
                        self.create_piece_clicked.set(true);
                    }
                }
            },
        );
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
    if string.len() <= num_chars * 2 + 3 {
        string.to_string()
    } else {
        format!(
            "{}...{}",
            &string[..num_chars],
            &string[string.len() - num_chars..]
        )
    }
}
