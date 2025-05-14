use macroquad::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorNode, OscillatorType};

#[derive(Debug, Clone)]
struct Rectangle {
    position: Vec2,
    size: Vec2,
}

impl Rectangle {
    fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x &&
        point.x <= self.position.x + self.size.x &&
        point.y >= self.position.y &&
        point.y <= self.position.y + self.size.y
    }

    fn draw(&self, color: Color) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, color);
    }
}

#[derive(Default)]
struct DragState {
    active: bool,
    offset: Vec2,
    start_x: f32,
}

impl DragState {
    fn start(&mut self, click_pos: Vec2, rect_pos: Vec2) {
        self.active = true;
        self.offset = click_pos - rect_pos;
        self.start_x = click_pos.x;
    }

    fn stop(&mut self) {
        self.active = false;
    }

    fn horizontal_drag(&self, current_x: f32) -> f32 {
        current_x - self.start_x
    }
}

struct GameState {
    rectangle: Rectangle,
    drag_state: DragState,
    audio_ctx: AudioContext,
    oscillator: Option<OscillatorNode>,
    base_freq: f32,
}

impl GameState {
    fn new() -> Result<Self, JsValue> {
        let audio_ctx = AudioContext::new()?;
        Ok(Self {
            rectangle: Rectangle {
                position: vec2(100.0, 100.0),
                size: vec2(150.0, 100.0),
            },
            drag_state: DragState::default(),
            audio_ctx,
            oscillator: None,
            base_freq: 440.0,
        })
    }

    fn handle_input(&mut self, mouse_pos: Vec2) {
        match (
            is_mouse_button_pressed(MouseButton::Left),
            is_mouse_button_released(MouseButton::Left),
        ) {
            (true, _) if self.rectangle.contains(mouse_pos) => {
                self.start_drag(mouse_pos);
            },
            (_, true) => {
                self.stop_drag();
            },
            _ => (),
        }

        if self.drag_state.active {
            self.update_drag(mouse_pos);
        }
    }

    fn start_drag(&mut self, mouse_pos: Vec2) {
        self.drag_state.start(mouse_pos, self.rectangle.position);
        self.create_oscillator().expect("Failed to start sound");
    }

    fn stop_drag(&mut self) {
        self.drag_state.stop();
        self.cleanup_oscillator();
    }

    fn update_drag(&mut self, mouse_pos: Vec2) {
        // Update rectangle position
        self.rectangle.position = mouse_pos - self.drag_state.offset;

        // Calculate frequency based on horizontal drag distance
        let delta_x = self.drag_state.horizontal_drag(mouse_pos.x);
        let new_freq = (self.base_freq + delta_x * 2.0).clamp(20.0, 20000.0);

        // Update oscillator frequency
        if let Some(osc) = &self.oscillator {
            osc.frequency().set_value(new_freq);
        }
    }

    fn create_oscillator(&mut self) -> Result<(), JsValue> {
        let oscillator = self.audio_ctx.create_oscillator()?;
        oscillator.set_type(OscillatorType::Sine);
        oscillator.frequency().set_value(self.base_freq);
        oscillator.connect_with_audio_node(&self.audio_ctx.destination())?;
        oscillator.start()?;
        self.oscillator = Some(oscillator);
        Ok(())
    }

    fn cleanup_oscillator(&mut self) {
        if let Some(osc) = self.oscillator.take() {
            let _ = osc.stop();
        }
    }

    fn draw(&self) {
        self.rectangle.draw(RED);
    }
}

#[macroquad::main("Drag Sound Demo")]
async fn main() {
    let mut game_state = GameState::new().expect("Failed to initialize audio");

    loop {
        clear_background(BLACK);
        let mouse_pos = mouse_position().into();

        game_state.handle_input(mouse_pos);
        game_state.draw();

        next_frame().await;
    }
}
