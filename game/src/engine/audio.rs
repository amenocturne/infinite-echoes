use web_sys::{AudioContext, GainNode, OscillatorNode};

pub struct AudioEngine {
    audio_context: AudioContext,
    osc_pool: Vec<OscillatorNode>,
}

pub struct OscillatorEngine {
    osc: OscillatorNode,
    gain: GainNode,
}
