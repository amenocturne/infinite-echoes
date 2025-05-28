use web_sys::{AudioContext, GainNode, OscillatorNode, OscillatorType};

use crate::core::GameTime;
use crate::engine::errors::{GameError, GameResult};
use crate::nodes::audio_graph::AudioGraph;
use crate::nodes::oscillator::WaveShape;

pub struct AudioEngine {
    audio_context: AudioContext,
}
impl AudioEngine {
    pub fn new() -> GameResult<AudioEngine> {
        let audio_context =
            AudioContext::new().map_err(GameError::js("Could not construct AudioContext"))?;
        Ok(AudioEngine { audio_context })
    }

    pub fn interpret_graph(&self, audio_graph: &AudioGraph) -> GameResult<()> {
        let note_generator = &audio_graph.note_generator;
        let oscillator = &audio_graph.oscillator; // TODO:
        let audio_context = &self.audio_context;

        let bpm = 120;
        // let loop_length_secs = note_generator.loop_length.to_seconds(bpm); // TODO:

        let now = audio_context.current_time();

        for note_event in &note_generator.notes {
            let freq = note_event.note.to_frequancy();
            let start = now + note_event.start.to_seconds(bpm);
            let duration = note_event.duration.to_seconds(bpm);

            let osc = GameOscillator::new(&self.audio_context, oscillator.wave_shape)?;
            osc.play(&self.audio_context, freq, start, duration)?;
        }
        Ok(())
    }
}

pub struct GameOscillator {
    osc: OscillatorNode,
    gain: GainNode,
    wave_shape: WaveShape,
}

impl GameOscillator {
    fn new(audio_context: &AudioContext, wave_shape: WaveShape) -> GameResult<GameOscillator> {
        let osc = audio_context
            .create_oscillator()
            .map_err(GameError::js("Could not create oscillator"))?;
        let gain = audio_context
            .create_gain()
            .map_err(GameError::js("Coult not create gain node"))?;
        Ok(GameOscillator {
            osc,
            gain,
            wave_shape,
        })
    }

    fn play(
        &self,
        audio_context: &AudioContext,
        frequency: f32,
        start: GameTime,
        duration: GameTime,
    ) -> GameResult<()> {
        let wave = match self.wave_shape {
            WaveShape::Sine => OscillatorType::Sine,
            WaveShape::Square => OscillatorType::Square,
        };
        self.osc.set_type(wave);
        self.osc.frequency().set_value(frequency);
        self.osc
            .connect_with_audio_node(&audio_context.destination())
            .map_err(GameError::js("Could not connect audio node to destination"))?;

        let start_time = start as f64;

        self.osc
            .start_with_when(start_time)
            .map_err(GameError::js("Could not start audio"))?;
        self.osc
            .stop_with_when(start_time + duration as f64)
            .map_err(GameError::js("Couldn't schedule stop"))?;
        Ok(())
    }
}
