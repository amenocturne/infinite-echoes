use crate::render::{Render, RenderAudio};
use std::cell::{Cell, RefCell};
use wasm_bindgen::JsValue;
use web_sys::AudioContext;
use web_sys::OscillatorNode;

use crate::render::rectangle::Rectangle;
use web_sys::OscillatorType;

pub enum OscillatorState {
    On,
    Off,
}

pub struct Oscillator {
    pub state: OscillatorState,
    pub frequency: Cell<f32>,
    pub wave: OscillatorType,
    pub rectangle: Rectangle,
    audio_node: RefCell<Option<OscillatorNode>>,
    has_started: Cell<bool>,
}

impl Oscillator {
    pub fn new(
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

    pub fn set_frequency(&self, frequency: f32) {
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
