use std::cell::RefCell;

use macroquad::color::GREEN;
use macroquad::color::WHITE;
use macroquad::math::vec2;

use crate::render::shapes::Shape;

use super::audio_node::AudioNode;
use super::audio_node::DisplayedAudioNode;

pub struct Oscillator {
    pub wave_shape: WaveShape,
    displayed_audio_node: RefCell<DisplayedAudioNode>,
}

#[derive(Clone, Copy)]
pub enum WaveShape {
    Sine,
    Square,
}

impl Oscillator {
    pub fn new(wave_shape: WaveShape) -> Oscillator {
        let displayed = DisplayedAudioNode::new(
            vec2(200.0, 200.0),
            vec2(50.0, 50.0),
            WHITE,
            GREEN,
            Shape::SineWave,
        );
        Oscillator {
            wave_shape,
            displayed_audio_node: RefCell::new(displayed),
        }
    }
}

impl AudioNode for Oscillator {
    fn as_displayed(&self) -> &RefCell<DisplayedAudioNode> {
        &self.displayed_audio_node
    }
}
