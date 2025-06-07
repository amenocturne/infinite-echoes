use std::cell::Cell;
use macroquad::math::{vec2, Vec2};
use macroquad::ui::{hash, root_ui, Skin};
use macroquad::prelude::*;

use crate::engine::errors::GameResult;
use crate::render::RenderCtx;

pub struct PieceLibraryWidget {
    is_visible: Cell<bool>,
    position: Vec2,
    size: Vec2,
    selected_address: Cell<Option<String>>,
}

impl PieceLibraryWidget {
    pub fn new() -> Self {
        Self {
            is_visible: Cell::new(false),
            position: vec2(0.5, 0.5),
            size: vec2(0.5, 0.7),
            selected_address: Cell::new(None),
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

    pub fn handle_load_selection(&self) -> Option<String> {
        self.selected_address.take()
    }

    pub fn render(
        &self,
        render_ctx: &RenderCtx,
        piece_addresses: &[String],
        is_loading: bool,
    ) -> GameResult<()> {
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
                .font_size(24)
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

        root_ui().push_skin(&dark_skin);
        root_ui().window(
            hash!(
                "library",
                render_ctx.screen_size.x.to_bits(),
                render_ctx.screen_size.y.to_bits()
            ),
            top_left,
            size,
            |ui| {
                ui.label(None, "Your Pieces");
                ui.separator();

                if piece_addresses.is_empty() && !is_loading {
                    ui.label(None, "You don't have any pieces yet.");
                } else {
                    for address in piece_addresses {
                        let short_addr = make_short(address, 8);
                        if ui.button(None, &*short_addr) {
                            self.selected_address.set(Some(address.clone()));
                        }
                    }
                }

                if is_loading {
                    ui.label(None, "Loading pieces...");
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
