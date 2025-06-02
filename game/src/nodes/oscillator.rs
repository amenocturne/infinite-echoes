#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Oscillator {
    pub wave_shape: WaveShape,
}

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
pub enum WaveShape {
    Sine,
    Square,
}

impl Oscillator {
    pub fn new(wave_shape: WaveShape) -> Oscillator {
        Oscillator { wave_shape }
    }
}
