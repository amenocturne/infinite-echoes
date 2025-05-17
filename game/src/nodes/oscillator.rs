pub struct Oscillator {
    pub wave_shape: WaveShape,
}

pub enum WaveShape {
    Sine,
}

impl Oscillator {
    pub fn new(wave_shape: WaveShape) -> Oscillator {
        Oscillator { wave_shape }
    }
}
