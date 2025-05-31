use std::cell::Cell;
use std::cell::RefCell;

use web_sys::AudioContext;
use web_sys::GainNode;
use web_sys::OscillatorNode;
use web_sys::OscillatorType;

use crate::core::GameTime;
use crate::engine::errors::GameError;
use crate::engine::errors::GameResult;
use crate::nodes::audio_graph::AudioGraph;
use crate::nodes::note_generator::MusicTime;
use crate::nodes::note_generator::NoteEvent;
use crate::nodes::oscillator::WaveShape;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AudioState {
    NotPlaying,
    Playing,
}

pub struct AudioEngine {
    audio_context: AudioContext,
    oscillators: Vec<RefCell<GameOscillator>>,
    state: Cell<AudioState>,
}
impl AudioEngine {
    pub fn new() -> GameResult<AudioEngine> {
        let audio_context =
            AudioContext::new().map_err(GameError::js("Could not construct AudioContext"))?;
        Ok(AudioEngine {
            audio_context,
            oscillators: vec![],
            state: Cell::new(AudioState::NotPlaying),
        })
    }

    pub fn is_playing(&self) -> bool {
        self.state.get() == AudioState::Playing
    }

    pub fn stop_all(&mut self) -> GameResult<()> {
        self.state.set(AudioState::NotPlaying);
        for osc in &self.oscillators {
            osc.borrow_mut().stop_immediate()?;
        }
        self.oscillators = vec![];
        Ok(())
    }

    pub fn interpret_graph(
        &mut self,
        bpm: u32,
        times: u32,
        audio_graph: &AudioGraph,
    ) -> GameResult<()> {
        self.state.set(AudioState::Playing);
        let note_generators = &audio_graph.note_generators();
        let oscillator = &audio_graph
            .oscillator()
            .ok_or(GameError::msg("Invalid garph: no oscillator found"))?;

        let when = self.audio_context.current_time();

        let mut notes = vec![];
        let mut acc_loop_start: MusicTime = MusicTime::new(0);
        for ng in note_generators {
            for ne in &ng.notes {
                notes.push(ne.shifted(acc_loop_start));
            }
            acc_loop_start = acc_loop_start + ng.loop_length;
        }

        let mut notes_repeated: Vec<NoteEvent> = vec![];
        for i in 0..times {
            let shifted_notes: Vec<NoteEvent> = notes
                .iter()
                .map(|n| n.shifted(audio_graph.loop_length() * i))
                .collect();
            for sn in shifted_notes {
                notes_repeated.push(sn);
            }
        }

        for note_event in notes_repeated {
            let freq = note_event.note.to_frequancy();
            let start = when + note_event.start.to_seconds(bpm);
            let duration = note_event.duration.to_seconds(bpm);

            let osc = GameOscillator::new(&self.audio_context, oscillator.wave_shape)?;
            osc.play(&self.audio_context, freq, start, duration)?;
            self.oscillators.push(RefCell::new(osc));
        }
        Ok(())
    }
}

pub struct GameOscillator {
    osc: OscillatorNode,
    gain: GainNode,
    wave_shape: WaveShape,
    is_stopped: bool,
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
            is_stopped: false,
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

        // Connect oscillator -> gain -> destination instead of direct connection
        self.osc
            .connect_with_audio_node(&self.gain)
            .map_err(GameError::js("Could not connect oscillator to gain"))?;
        self.gain
            .connect_with_audio_node(&audio_context.destination())
            .map_err(GameError::js("Could not connect gain to destination"))?;

        let start_time = start as f64;
        let end_time = start_time + duration as f64;

        // Attack and release times to remove clicks
        let attack_time = 0.001; // 10ms attack
        let release_time = 0.002; // 20ms release

        // Set initial gain to 0 to avoid clicks
        self.gain
            .gain()
            .set_value_at_time(0.0, start_time)
            .map_err(GameError::js("Could not set initial gain"))?;

        // Attack: ramp from 0 to 1 over attack_time
        self.gain
            .gain()
            .linear_ramp_to_value_at_time(1.0, start_time + attack_time)
            .map_err(GameError::js("Could not schedule attack ramp"))?;

        // Release: ramp from 1 to 0 over release_time before stopping
        let release_start = end_time - release_time;
        self.gain
            .gain()
            .set_value_at_time(1.0, release_start)
            .map_err(GameError::js("Could not set release start gain"))?;
        self.gain
            .gain()
            .linear_ramp_to_value_at_time(0.0, end_time)
            .map_err(GameError::js("Could not schedule release ramp"))?;

        self.osc
            .start_with_when(start_time)
            .map_err(GameError::js("Could not start audio"))?;
        self.osc
            .stop_with_when(end_time)
            .map_err(GameError::js("Couldn't schedule stop"))?;
        Ok(())
    }

    fn stop_immediate(&mut self) -> GameResult<()> {
        self.osc
            .stop()
            .map_err(GameError::js("Could not stop oscillator"))?;

        self.is_stopped = true;

        Ok(())
    }

    fn stop_at(&mut self, when: GameTime) -> GameResult<()> {
        self.osc
            .stop_with_when(when as f64)
            .map_err(GameError::js("Could not schedule stop"))?;

        self.is_stopped = true; // TODO: maybe should be also delayed
        Ok(())
    }
}
