use std::cell::Cell;
use std::cell::RefCell;

use crate::core::GameTime;
use crate::engine::errors::GameError;
use crate::engine::errors::GameResult;
use crate::nodes::audio_effect::AudioEffect;
use crate::nodes::audio_effect::DistortionCurve;
use crate::nodes::audio_effect::DistortionParameters;
use crate::nodes::audio_effect::FilterParameters;
use crate::nodes::audio_effect::ReverbParameters;
use crate::nodes::audio_graph::AudioGraph;
use crate::nodes::oscillator::WaveShape;
use web_sys::js_sys::Float32Array;
use web_sys::AudioContext;
use web_sys::AudioNode as WebAudioNode;
use web_sys::BiquadFilterNode;
use web_sys::ConvolverNode;
use web_sys::GainNode;
use web_sys::OscillatorNode;
use web_sys::OscillatorType;
use web_sys::OverSampleType;
use web_sys::WaveShaperNode;

use super::game_config::AudioConfig;
use crate::core::random;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AudioState {
    NotPlaying,
    Playing,
}

pub struct AudioEngine {
    audio_context: AudioContext,
    master_gain: GainNode,
    oscillators: Vec<RefCell<GameOscillator>>,
    effects: Vec<RefCell<Box<dyn AudioEffectNode>>>,
    state: Cell<AudioState>,
}

impl AudioEngine {
    pub fn new() -> GameResult<AudioEngine> {
        let audio_context =
            AudioContext::new().map_err(GameError::js("Could not construct AudioContext"))?;

        let master_gain = audio_context
            .create_gain()
            .map_err(GameError::js("Could not create master gain node"))?;

        master_gain
            .connect_with_audio_node(&audio_context.destination())
            .map_err(GameError::js(
                "Could not connect master gain to destination",
            ))?;

        master_gain.gain().set_value(1.0);

        Ok(AudioEngine {
            audio_context,
            master_gain,
            oscillators: vec![],
            effects: vec![],
            state: Cell::new(AudioState::NotPlaying),
        })
    }

    /// Set the volume (0.0 to 1.0)
    pub fn set_volume(&self, volume: f32) {
        let clamped_volume = volume.clamp(0.0, 1.0);
        let exponential_volume = clamped_volume.sqrt();
        self.master_gain.gain().set_value(exponential_volume);
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
        self.effects = vec![];
        Ok(())
    }

    pub fn interpret_graph(
        &mut self,
        bpm: u32,
        audio_graph: &AudioGraph,
        audio_config: &AudioConfig,
    ) -> GameResult<()> {
        self.state.set(AudioState::Playing);
        let oscillator = &audio_graph
            .oscillator()
            .ok_or(GameError::msg("Invalid graph: no oscillator found"))?;
        let audio_effects = audio_graph.audio_effects();

        let when = self.audio_context.current_time();

        let mut effect_nodes: Vec<Box<dyn AudioEffectNode>> = vec![];
        for effect in &audio_effects {
            match effect {
                AudioEffect::Filter(filter_params) => {
                    let filter = GameFilter::new(&self.audio_context, filter_params)?;
                    effect_nodes.push(Box::new(filter));
                }
                AudioEffect::Distortion(distortion_params) => {
                    let distortion = GameDistortion::new(&self.audio_context, distortion_params)?;
                    effect_nodes.push(Box::new(distortion));
                }
                AudioEffect::Reverb(reverb_params) => {
                    let reverb = GameReverb::new(&self.audio_context, reverb_params)?;
                    effect_nodes.push(Box::new(reverb));
                }
            }
        }

        let oscillator_destination: WebAudioNode = if effect_nodes.is_empty() {
            self.master_gain.clone().into()
        } else {
            for i in 1..effect_nodes.len() {
                let prev_output = effect_nodes[i - 1].get_output_node();
                let current_input = effect_nodes[i].get_input_node();

                prev_output
                    .as_ref()
                    .connect_with_audio_node(current_input.as_ref())
                    .map_err(GameError::js("Could not connect effects in chain"))?;
            }

            if let Some(last_effect) = effect_nodes.last() {
                last_effect
                    .get_output_node()
                    .as_ref()
                    .connect_with_audio_node(&self.master_gain)
                    .map_err(GameError::js(
                        "Could not connect final effect to master gain",
                    ))?;
            }

            effect_nodes[0].get_input_node().as_ref().clone()
        };

        for effect in effect_nodes {
            self.effects.push(RefCell::new(effect));
        }

        let generator = audio_graph.process_note_generators();

        let loop_length_seconds = generator.loop_length.to_seconds(bpm);

        let loops_to_schedule =
            (audio_config.max_schedule_ahead / loop_length_seconds).ceil() as i32;

        let mut notes_repeated = vec![];
        for i in 0..loops_to_schedule {
            let shifted_notes: Vec<_> = generator
                .notes
                .iter()
                .map(|n: &_| n.shifted(generator.loop_length * i as u32))
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
            osc.play_with_destination(
                &oscillator_destination,
                freq,
                start,
                duration,
                audio_config,
            )?;
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
            .map_err(GameError::js("Could not create gain node"))?;
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
        audio_config: &AudioConfig,
    ) -> GameResult<()> {
        self.play_with_destination(
            &audio_context.destination().into(),
            frequency,
            start,
            duration,
            audio_config,
        )
    }

    fn play_with_destination(
        &self,
        destination: &WebAudioNode,
        frequency: f32,
        start: GameTime,
        duration: GameTime,
        audio_config: &AudioConfig,
    ) -> GameResult<()> {
        let wave = match self.wave_shape {
            WaveShape::Sine => OscillatorType::Sine,
            WaveShape::Square => OscillatorType::Square,
        };
        self.osc.set_type(wave);
        self.osc.frequency().set_value(frequency);

        self.osc
            .connect_with_audio_node(&self.gain)
            .map_err(GameError::js("Could not connect oscillator to gain"))?;
        self.gain
            .connect_with_audio_node(destination)
            .map_err(GameError::js("Could not connect gain to destination"))?;

        let start_time = start as f64;
        let end_time = start_time + duration as f64;

        let attack_time = audio_config.attack_time;
        let release_time = audio_config.release_time;

        self.gain
            .gain()
            .set_value_at_time(0.0, start_time)
            .map_err(GameError::js("Could not set initial gain"))?;

        self.gain
            .gain()
            .linear_ramp_to_value_at_time(audio_config.output_gain, start_time + attack_time)
            .map_err(GameError::js("Could not schedule attack ramp"))?;

        let release_start = end_time - release_time;
        self.gain
            .gain()
            .set_value_at_time(audio_config.output_gain, release_start)
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

    fn play_looping(
        &self,
        destination: &WebAudioNode,
        frequency: f32,
        start: GameTime,
        duration: GameTime,
        loop_period: GameTime,
        audio_config: &AudioConfig,
    ) -> GameResult<()> {
        let wave = match self.wave_shape {
            WaveShape::Sine => OscillatorType::Sine,
            WaveShape::Square => OscillatorType::Square,
        };
        self.osc.set_type(wave);
        self.osc.frequency().set_value(frequency);

        self.osc
            .connect_with_audio_node(&self.gain)
            .map_err(GameError::js("Could not connect oscillator to gain"))?;
        self.gain
            .connect_with_audio_node(destination)
            .map_err(GameError::js("Could not connect gain to destination"))?;

        let start_time = start as f64;
        let note_duration = duration as f64;
        let loop_period_secs = loop_period as f64;

        let attack_time = audio_config.attack_time;
        let release_time = audio_config.release_time;

        self.osc
            .start_with_when(start_time)
            .map_err(GameError::js("Could not start audio"))?;

        let loops_to_schedule = (audio_config.max_schedule_ahead / loop_period_secs).ceil() as i32;

        for i in 0..loops_to_schedule {
            let loop_start = start_time + (loop_period_secs * i as f64);
            let note_start = loop_start;
            let note_end = note_start + note_duration;

            self.gain
                .gain()
                .set_value_at_time(0.0, note_start)
                .map_err(GameError::js("Could not set initial gain"))?;

            self.gain
                .gain()
                .linear_ramp_to_value_at_time(audio_config.output_gain, note_start + attack_time)
                .map_err(GameError::js("Could not schedule attack ramp"))?;

            let release_start = note_end - release_time;
            self.gain
                .gain()
                .set_value_at_time(audio_config.output_gain, release_start)
                .map_err(GameError::js("Could not set release start gain"))?;
            self.gain
                .gain()
                .linear_ramp_to_value_at_time(0.0, note_end)
                .map_err(GameError::js("Could not schedule release ramp"))?;
        }

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
        self.is_stopped = true;
        Ok(())
    }
}

pub struct GameFilter {
    filter: BiquadFilterNode,
    parameters: FilterParameters,
}

impl GameFilter {
    fn new(audio_context: &AudioContext, params: &FilterParameters) -> GameResult<GameFilter> {
        let filter = audio_context
            .create_biquad_filter()
            .map_err(GameError::js("Could not create biquad filter"))?;

        filter.set_type(params.filter_type.to_biquad_type());
        filter.frequency().set_value(params.frequency);
        filter.q().set_value(params.q);

        Ok(GameFilter {
            filter,
            parameters: params.clone(),
        })
    }

    fn get_node(&self) -> &BiquadFilterNode {
        &self.filter
    }
}

pub struct GameDistortion {
    input_gain: GainNode,
    wave_shaper: WaveShaperNode,
    output_gain: GainNode,
    parameters: DistortionParameters,
}

impl GameDistortion {
    fn new(
        audio_context: &AudioContext,
        params: &DistortionParameters,
    ) -> GameResult<GameDistortion> {
        let input_gain = audio_context.create_gain().map_err(GameError::js(
            "Could not create input gain node for distortion",
        ))?;

        let wave_shaper = audio_context
            .create_wave_shaper()
            .map_err(GameError::js("Could not create wave shaper"))?;

        let output_gain = audio_context.create_gain().map_err(GameError::js(
            "Could not create output gain node for distortion",
        ))?;

        input_gain.gain().set_value(1.0 + params.amount * 10.0);

        let samples = 44100;
        let curve: Vec<f32> = (0..samples)
            .map(|i| {
                let x = (i as f32 / (samples - 1) as f32) * 2.0 - 1.0;

                match params.curve_type {
                    DistortionCurve::SoftClip => x.tanh() * (1.0 + params.amount * 0.5),
                    DistortionCurve::HardClip => {
                        let k = params.amount * 100.0;
                        let deg = std::f32::consts::PI / 180.0;

                        if x.abs() < 0.001 {
                            x
                        } else {
                            let distorted =
                                ((3.0 + k) * x * 20.0 * deg) / (std::f32::consts::PI + k * x.abs());

                            distorted.max(-1.0).min(1.0)
                        }
                    }
                }
            })
            .collect();

        wave_shaper.set_curve_opt_f32_array(Some(&Float32Array::from(curve.as_slice())));

        wave_shaper.set_oversample(OverSampleType::N2x);

        let compensation_gain = match params.curve_type {
            DistortionCurve::SoftClip => 1.0 / (1.0 + params.amount * 0.5),
            DistortionCurve::HardClip => 0.3 / (1.0 + params.amount),
        };
        output_gain.gain().set_value(compensation_gain);

        input_gain
            .connect_with_audio_node(&wave_shaper)
            .map_err(GameError::js("Could not connect input gain to wave shaper"))?;

        wave_shaper
            .connect_with_audio_node(&output_gain)
            .map_err(GameError::js(
                "Could not connect wave shaper to output gain",
            ))?;

        Ok(GameDistortion {
            input_gain,
            wave_shaper,
            output_gain,
            parameters: params.clone(),
        })
    }

    fn get_input_node(&self) -> &GainNode {
        &self.input_gain
    }

    fn get_output_node(&self) -> &GainNode {
        &self.output_gain
    }
}

pub struct GameReverb {
    input_gain: GainNode,
    convolver: ConvolverNode,
    output_gain: GainNode,
    parameters: ReverbParameters,
}

impl GameReverb {
    fn new(audio_context: &AudioContext, params: &ReverbParameters) -> GameResult<GameReverb> {
        let input_gain = audio_context
            .create_gain()
            .map_err(GameError::js("Could not create input gain for reverb"))?;
        let convolver = audio_context
            .create_convolver()
            .map_err(GameError::js("Could not create convolver node"))?;
        let output_gain = audio_context
            .create_gain()
            .map_err(GameError::js("Could not create output gain for reverb"))?;

        let sample_rate = audio_context.sample_rate();
        let length = (sample_rate * params.decay_time) as usize;
        let mut impulse_data = vec![0.0f32; length];

        let pre_delay_samples = (sample_rate * 0.02) as usize;
        let decay_exponent = 2.0;

        for i in 0..length {
            let mut sample_value = 0.0f32;

            if i >= pre_delay_samples {
                let normalized_time =
                    (i - pre_delay_samples) as f32 / (length - pre_delay_samples) as f32;
                let decay = (1.0 - normalized_time).powf(decay_exponent);
                sample_value = (random() * 2.0 - 1.0) * decay;
            }

            impulse_data[i] = sample_value;
        }

        let impulse_buffer = audio_context
            .create_buffer(1, length as u32, sample_rate)
            .map_err(GameError::js("Could not create audio buffer for impulse"))?;

        impulse_buffer
            .copy_to_channel(&impulse_data, 0)
            .map_err(GameError::js("Could not copy impulse data to buffer"))?;

        convolver.set_buffer(Some(&impulse_buffer));

        input_gain.gain().set_value(params.dry_level);
        output_gain.gain().set_value(params.wet_level);

        input_gain
            .connect_with_audio_node(&convolver)
            .map_err(GameError::js("Could not connect input gain to convolver"))?;
        convolver
            .connect_with_audio_node(&output_gain)
            .map_err(GameError::js("Could not connect convolver to output gain"))?;

        Ok(GameReverb {
            input_gain,
            convolver,
            output_gain,
            parameters: params.clone(),
        })
    }

    fn get_input_node(&self) -> &GainNode {
        &self.input_gain
    }

    fn get_output_node(&self) -> &GainNode {
        &self.output_gain
    }
}

trait AudioEffectNode {
    fn get_input_node(&self) -> &dyn AsRef<WebAudioNode>;
    fn get_output_node(&self) -> &dyn AsRef<WebAudioNode>;
}

impl AudioEffectNode for GameFilter {
    fn get_input_node(&self) -> &dyn AsRef<WebAudioNode> {
        &self.filter
    }

    fn get_output_node(&self) -> &dyn AsRef<WebAudioNode> {
        &self.filter
    }
}

impl AudioEffectNode for GameDistortion {
    fn get_input_node(&self) -> &dyn AsRef<WebAudioNode> {
        &self.input_gain
    }

    fn get_output_node(&self) -> &dyn AsRef<WebAudioNode> {
        &self.output_gain
    }
}

impl AudioEffectNode for GameReverb {
    fn get_input_node(&self) -> &dyn AsRef<WebAudioNode> {
        &self.input_gain
    }

    fn get_output_node(&self) -> &dyn AsRef<WebAudioNode> {
        &self.output_gain
    }
}
