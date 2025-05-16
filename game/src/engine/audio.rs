use web_sys::{AudioContext, GainNode, OscillatorNode, OscillatorType};

use crate::{
    errors::{GameError, GameResult},
    nodes::note_generator::Note,
};

pub struct AudioEngine {
    audio_context: AudioContext,
}
impl AudioEngine {
    pub fn new() -> GameResult<AudioEngine> {
        let audio_context =
            AudioContext::new().map_err(GameError::js("Could not construct AudioContext"))?;
        Ok(AudioEngine { audio_context })
    }

    pub fn play_freq(&self, frequency: f32, duration: f32) -> GameResult<()> {
        let osc = GameOscillator::new(&self.audio_context)?;
        osc.play(&self.audio_context, 0.0, frequency, duration)
    }

    pub fn play_notes(&self, notes: &[Note]) -> GameResult<()> {
        for note in notes {
            let osc = GameOscillator::new(&self.audio_context)?;
            osc.play(
                &self.audio_context,
                note.to_frequancy(),
                note.position.start,
                note.position.duration,
            )?;
        }
        Ok(())
    }
}

pub struct GameOscillator {
    osc: OscillatorNode,
    gain: GainNode,
}

impl GameOscillator {
    fn new(audio_context: &AudioContext) -> GameResult<GameOscillator> {
        let osc = audio_context
            .create_oscillator()
            .map_err(GameError::js("Could not create oscillator"))?;
        let gain = audio_context
            .create_gain()
            .map_err(GameError::js("Coult not create gain node"))?;
        Ok(GameOscillator { osc, gain })
    }

    fn play(
        &self,
        audio_context: &AudioContext,
        frequency: f32,
        start: f32,
        duration: f32,
    ) -> GameResult<()> {
        self.osc.set_type(OscillatorType::Sine);
        self.osc.frequency().set_value(frequency);
        self.osc
            .connect_with_audio_node(&audio_context.destination())
            .map_err(GameError::js("Could not connect audio node to destination"))?;

        let now = audio_context.current_time();
        let start_time = now + start as f64;

        self.osc
            .start_with_when(start_time)
            .map_err(GameError::js("Could not start audio"))?;
        self.osc
            .stop_with_when(start_time + duration as f64)
            .map_err(GameError::js("Couldn't schedule stop"))?;
        Ok(())
    }
}
