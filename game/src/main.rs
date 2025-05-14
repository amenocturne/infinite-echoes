use macroquad::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

#[wasm_bindgen]
pub fn create_oscillator() -> Result<(), JsValue> {
    let ctx = AudioContext::new()?;
    let oscillator = ctx.create_oscillator()?;
    oscillator.set_type(OscillatorType::Sine);
    oscillator.frequency().set_value(440.0);
    oscillator.connect_with_audio_node(&ctx.destination())?;
    oscillator.start()?;
    Ok(())
}

#[macroquad::main("Movable Rectangle")]
async fn main() {
    let mut rect_pos = vec2(100.0, 100.0);
    let rect_size = vec2(150.0, 100.0);
    let mut dragging = false;
    let mut drag_offset = vec2(0.0, 0.0);

    let _ = create_oscillator();

    loop {
        clear_background(BLACK);
        let mouse_pos: Vec2 = mouse_position().into();

        if is_mouse_button_pressed(MouseButton::Left) {
            if mouse_pos.x >= rect_pos.x
                && mouse_pos.x <= rect_pos.x + rect_size.x
                && mouse_pos.y >= rect_pos.y
                && mouse_pos.y <= rect_pos.y + rect_size.y
            {
                dragging = true;
                drag_offset = mouse_pos - rect_pos;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragging = false;
        }

        if dragging {
            rect_pos = mouse_pos - drag_offset;
        }

        draw_rectangle(rect_pos.x, rect_pos.y, rect_size.x, rect_size.y, RED);
        next_frame().await;
    }
}
