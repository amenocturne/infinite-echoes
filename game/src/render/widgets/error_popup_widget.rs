use std::cell::Cell;
use macroquad::math::{vec2, Vec2};
use macroquad::ui::{hash, root_ui, Skin};
use macroquad::prelude::*;

use crate::engine::errors::GameResult;
use crate::render::RenderCtx;

pub struct ErrorPopupWidget {
    is_visible: Cell<bool>,
    message: Cell<Option<String>>,
    position: Vec2,
    size: Vec2,
}

impl ErrorPopupWidget {
    pub fn new() -> Self {
        Self {
            is_visible: Cell::new(false),
            message: Cell::new(None),
            position: vec2(0.5, 0.5),
            size: vec2(0.4, 0.25),
        }
    }

    pub fn show(&self, message: String) {
        self.message.set(Some(message));
        self.is_visible.set(true);
    }

    pub fn hide(&self) {
        self.is_visible.set(false);
        self.message.set(None);
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible.get()
    }

    pub fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        if !self.is_visible.get() {
            return Ok(());
        }

        let message = match self.message.take() {
            Some(msg) => {
                self.message.set(Some(msg.clone()));
                msg
            }
            None => return Ok(()),
        };

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
                .margin(RectOffset::new(10., 10., 10., 10.))
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
                "error",
                render_ctx.screen_size.x.to_bits(),
                render_ctx.screen_size.y.to_bits()
            ),
            top_left,
            size,
            |ui| {
                ui.label(None, "Error");
                ui.separator();
                ui.label(None, &message);

                let button_size = ui.calc_size("OK");

                // The window has a margin of 10px on each side.
                // The content area size is `size - vec2(20.0, 20.0)`.
                // The button position is relative to this content area.
                let margin_x = 20.0;
                let margin_y = 20.0;
                let content_area_size = size - vec2(margin_x, margin_y);

                let button_x = (content_area_size.x - button_size.x) / 2.0;
                let button_y = content_area_size.y - button_size.y - 10.0; // 10px padding from bottom

                if ui.button(Some(vec2(button_x, button_y)), "OK") {
                    self.hide();
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
