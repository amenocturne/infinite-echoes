use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

use macroquad::prelude::*;

use wasm_bindgen::JsValue;
use web_sys::{AudioContext, OscillatorNode, OscillatorType};

trait Render {
    fn render(&self) -> ();
}

trait RenderAudio {
    fn render_audio(&self, audio_context: &AudioContext) -> Result<(), JsValue>;
}

struct Rectangle {
    position: Vec2,
    size: Vec2,
    color: Color,
}

impl Rectangle {
    fn new(position: Vec2, size: Vec2, color: Color) -> Rectangle {
        Rectangle {
            position,
            size,
            color,
        }
    }
}

impl Render for Rectangle {
    fn render(&self) -> () {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }
}

struct GameState {
    audio_context: AudioContext,
    oscillator: Oscillator,
    is_dragging: bool,
}

impl GameState {
    fn new(audio_context: AudioContext, oscillator: Oscillator) -> GameState {
        GameState {
            audio_context,
            oscillator,
            is_dragging: false,
        }
    }
}

impl Render for GameState {
    fn render(&self) -> () {
        self.oscillator.rectangle.render();
    }
}

impl RenderAudio for GameState {
    fn render_audio(&self, audio_context: &AudioContext) -> Result<(), JsValue> {
        self.oscillator.render_audio(audio_context)
    }
}

enum OscillatorState {
    On,
    Off,
}

struct Oscillator {
    state: OscillatorState,
    frequency: Cell<f32>,
    wave: OscillatorType,
    rectangle: Rectangle,
    audio_node: RefCell<Option<OscillatorNode>>,
    has_started: Cell<bool>,
}

impl Oscillator {
    fn new(
        state: OscillatorState,
        frequency: f32,
        wave: OscillatorType,
        rectangle: Rectangle,
    ) -> Oscillator {
        let audio_node = RefCell::new(None);
        let has_started = Cell::new(false);
        let frequency = Cell::new(frequency);
        Oscillator {
            state,
            frequency,
            wave,
            rectangle,
            audio_node,
            has_started,
        }
    }

    fn set_frequency(&self, frequency: f32) {
        self.frequency.set(frequency);

        if let Some(node) = self.audio_node.borrow().as_ref() {
            node.frequency().set_value(frequency);
        }
    }
}

impl Render for Oscillator {
    fn render(&self) {
        self.rectangle.render()
    }
}

impl RenderAudio for Oscillator {
    fn render_audio(&self, audio_context: &AudioContext) -> Result<(), JsValue> {
        match self.state {
            OscillatorState::On => {
                if !self.has_started.get() {
                    let mut node_ref = self.audio_node.borrow_mut();
                    *node_ref = Some(audio_context.create_oscillator()?);
                    let node = node_ref.as_ref().unwrap();
                    node.set_type(self.wave);
                    node.frequency().set_value(self.frequency.get());
                    node.connect_with_audio_node(&audio_context.destination())?;
                    node.start()?;
                    self.has_started.set(true);
                }
                Ok(())
            }
            OscillatorState::Off => {
                if self.has_started.get() {
                    if let Some(node) = self.audio_node.borrow_mut().take() {
                        node.stop()?;
                    }
                    self.has_started.set(false);
                }
                Ok(())
            }
        }
    }
}

enum GameEvent {
    OscillatorStart,
    OscillatorStop,
    OscillatorDrag { mouse_pos: Vec2 },
    OscillatorSetFrequency { frequency: f32 },
    DraggingStart,
    DraggingStop,
}

fn process_event(game_state: &mut GameState, event: &GameEvent) {
    match event {
        GameEvent::OscillatorStart => game_state.oscillator.state = OscillatorState::On,
        GameEvent::OscillatorStop => game_state.oscillator.state = OscillatorState::Off,
        GameEvent::OscillatorDrag { mouse_pos } => {
            game_state.oscillator.rectangle.position =
                *mouse_pos - (0.5 * game_state.oscillator.rectangle.size)
        }
        GameEvent::OscillatorSetFrequency { frequency } => {
            game_state.oscillator.set_frequency(*frequency);
        }
        GameEvent::DraggingStart => game_state.is_dragging = true,
        GameEvent::DraggingStop => game_state.is_dragging = false,
    }
}

#[macroquad::main("Drag Sound Demo")]
async fn main() -> Result<(), JsValue> {
    let audio_context = AudioContext::new()?;
    let osc_rectangle = Rectangle::new(vec2(0.0, 0.0), vec2(50.0, 50.0), RED);
    let oscillator = Oscillator::new(
        OscillatorState::Off,
        440.0,
        OscillatorType::Sine,
        osc_rectangle,
    );

    let mut game_state = GameState::new(audio_context, oscillator);
    let mut event_queue: VecDeque<GameEvent> = VecDeque::new();

    loop {
        clear_background(BLACK);
        let mouse_pos: Vec2 = mouse_position().into();
        let width = screen_width();
        // let height = screen_height();

        // Input
        {
            let mut emit = |event: GameEvent| event_queue.push_back(event);

            if is_mouse_button_pressed(MouseButton::Left) {
                emit(GameEvent::OscillatorStart);
                emit(GameEvent::DraggingStart);
            }

            if is_mouse_button_released(MouseButton::Left) {
                emit(GameEvent::OscillatorStop);
                emit(GameEvent::DraggingStop);
            }

            // Additional events based on the state
            if game_state.is_dragging {
                emit(GameEvent::OscillatorDrag { mouse_pos });
                emit(GameEvent::OscillatorSetFrequency {
                    frequency: 20.0
                        * (10_000.0 / 20.0 as f64).powf((mouse_pos.x as f64) / width as f64) as f32,
                });
            }
        }

        // Event processing
        while let Some(event) = event_queue.pop_front() {
            process_event(&mut game_state, &event);
        }

        //Rendering
        game_state.render_audio(&game_state.audio_context)?;
        game_state.render();
        next_frame().await;
    }
}
