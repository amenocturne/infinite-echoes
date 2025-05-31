use std::cell::Cell;
use std::cell::RefCell;

use crate::core::GameTime;
use crate::engine::errors::GameError;
use crate::engine::errors::GameResult;
use crate::nodes::audio_effect::AudioEffect;
use crate::nodes::audio_effect::DistortionParameters;
use crate::nodes::audio_effect::FilterParameters;
use crate::nodes::audio_effect::FilterType;
use crate::nodes::audio_graph::AudioGraph;
use crate::nodes::note_generator::MusicTime;
use crate::nodes::oscillator::WaveShape;
use web_sys::AudioContext;
use web_sys::AudioNode as WebAudioNode;
use web_sys::BiquadFilterNode;
use web_sys::GainNode;
use web_sys::OscillatorNode;
use web_sys::OscillatorType;
use web_sys::WaveShaperNode;
use web_sys::OverSampleType;

use super::game_config::AudioConfig;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AudioState {
    NotPlaying,
    Playing,
}

pub struct AudioEngine {
    audio_context: AudioContext,
    oscillators: Vec<RefCell<GameOscillator>>,
    effects: Vec<RefCell<Box<dyn AudioEffectNode>>>,
    state: Cell<AudioState>,
}

impl AudioEngine {
    pub fn new() -> GameResult<AudioEngine> {
        let audio_context =
            AudioContext::new().map_err(GameError::js("Could not construct AudioContext"))?;
        Ok(AudioEngine {
            audio_context,
            oscillators: vec![],
            effects: vec![],
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
        let note_generators = &audio_graph.note_generators();
        let oscillator = &audio_graph
            .oscillator()
            .ok_or(GameError::msg("Invalid graph: no oscillator found"))?;
        let audio_effects = audio_graph.audio_effects();

        let when = self.audio_context.current_time();

        // Create effect chain
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
            }
        }

        // Determine the final destination and connect effect chain
        let oscillator_destination: WebAudioNode = if effect_nodes.is_empty() {
            self.audio_context.destination().into()
        } else {
            // Connect effects in series
            for i in 1..effect_nodes.len() {
                let prev_output = effect_nodes[i - 1].get_output_node();
                let current_input = effect_nodes[i].get_input_node();

                prev_output
                    .as_ref()
                    .connect_with_audio_node(current_input.as_ref())
                    .map_err(GameError::js("Could not connect effects in chain"))?;
            }

            // Connect the last effect to the destination
            if let Some(last_effect) = effect_nodes.last() {
                last_effect
                    .get_output_node()
                    .as_ref()
                    .connect_with_audio_node(&self.audio_context.destination())
                    .map_err(GameError::js(
                        "Could not connect final effect to destination",
                    ))?;
            }

            // Return the first effect's input as the destination for oscillators
            effect_nodes[0].get_input_node().as_ref().clone()
        };

        // Store effects to prevent them from being dropped
        for effect in effect_nodes {
            self.effects.push(RefCell::new(effect));
        }

        // Prepare notes
        let mut notes = vec![];
        let mut acc_loop_start: MusicTime = MusicTime::new(0);
        for ng in note_generators {
            for ne in &ng.notes {
                notes.push(ne.shifted(acc_loop_start));
            }
            acc_loop_start = acc_loop_start + ng.loop_length;
        }

        // Calculate total loop length in seconds
        let loop_length_seconds = audio_graph.loop_length().to_seconds(bpm);
        
        // Calculate how many loops to schedule based on max_schedule_ahead
        let loops_to_schedule = (audio_config.max_schedule_ahead / loop_length_seconds).ceil() as i32;
        
        // Create and schedule oscillators for many loops ahead
        let mut notes_repeated = vec![];
        for i in 0..loops_to_schedule {
            let shifted_notes: Vec<_> = notes
                .iter()
                .map(|n| n.shifted(audio_graph.loop_length() * i as u32))
                .collect();
            for sn in shifted_notes {
                notes_repeated.push(sn);
            }
        }

        // Create and play oscillators
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

        // Connect oscillator -> gain -> destination
        self.osc
            .connect_with_audio_node(&self.gain)
            .map_err(GameError::js("Could not connect oscillator to gain"))?;
        self.gain
            .connect_with_audio_node(destination)
            .map_err(GameError::js("Could not connect gain to destination"))?;

        let start_time = start as f64;
        let end_time = start_time + duration as f64;

        // Get attack and release times from config
        let attack_time = audio_config.attack_time;
        let release_time = audio_config.release_time;

        // Set initial gain to 0 to avoid clicks
        self.gain
            .gain()
            .set_value_at_time(0.0, start_time)
            .map_err(GameError::js("Could not set initial gain"))?;

        // Attack: ramp from 0 to output_gain over attack_time
        self.gain
            .gain()
            .linear_ramp_to_value_at_time(audio_config.output_gain, start_time + attack_time)
            .map_err(GameError::js("Could not schedule attack ramp"))?;

        // Release: ramp from output_gain to 0 over release_time before stopping
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

        // Connect oscillator -> gain -> destination
        self.osc
            .connect_with_audio_node(&self.gain)
            .map_err(GameError::js("Could not connect oscillator to gain"))?;
        self.gain
            .connect_with_audio_node(destination)
            .map_err(GameError::js("Could not connect gain to destination"))?;

        let start_time = start as f64;
        let note_duration = duration as f64;
        let loop_period_secs = loop_period as f64;
        
        // Get attack and release times from config
        let attack_time = audio_config.attack_time;
        let release_time = audio_config.release_time;
        
        // Start the oscillator - it will continue until explicitly stopped
        self.osc
            .start_with_when(start_time)
            .map_err(GameError::js("Could not start audio"))?;
            
        // Schedule a large number of loops ahead
        let loops_to_schedule = (audio_config.max_schedule_ahead / loop_period_secs).ceil() as i32;
        
        for i in 0..loops_to_schedule {
            let loop_start = start_time + (loop_period_secs * i as f64);
            let note_start = loop_start;
            let note_end = note_start + note_duration;
            
            // Set initial gain to 0 at the start of each note
            self.gain
                .gain()
                .set_value_at_time(0.0, note_start)
                .map_err(GameError::js("Could not set initial gain"))?;
                
            // Attack: ramp from 0 to output_gain over attack_time
            self.gain
                .gain()
                .linear_ramp_to_value_at_time(audio_config.output_gain, note_start + attack_time)
                .map_err(GameError::js("Could not schedule attack ramp"))?;
                
            // Release: ramp from output_gain to 0 over release_time before note end
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

        // Set gain for filters that support it
        if matches!(
            params.filter_type,
            FilterType::Peaking | FilterType::LowShelf | FilterType::HighShelf
        ) {
            filter.gain().set_value(params.gain);
        }

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

        // Set input gain to drive the signal into distortion
        // Amount parameter controls how hard we drive the signal
        input_gain.gain().set_value(1.0 + params.amount * 10.0);

        // Create distortion curve
        let samples = 44100;
        let mut curve: Vec<f32> = (0..samples)
            .map(|i| {
                let x = (i as f32 / (samples - 1) as f32) * 2.0 - 1.0; // Map to [-1, 1]

                // Create distortion using sigmoid formula (from MDN examples)
                let k = params.amount * 100.0; // Drive amount
                let deg = std::f32::consts::PI / 180.0;

                if x.abs() < 0.001 {
                    // Avoid division by zero near zero
                    x
                } else {
                    let distorted =
                        ((3.0 + k) * x * 20.0 * deg) / (std::f32::consts::PI + k * x.abs());

                    // Apply threshold as hard clipping
                    if distorted > params.threshold {
                        params.threshold
                    } else if distorted < -params.threshold {
                        -params.threshold
                    } else {
                        distorted
                    }
                }
            })
            .collect();

        // Create Float32Array for the curve
        wave_shaper.set_curve(Some(curve.as_mut_slice()));

        // Set oversample for better quality
        wave_shaper.set_oversample(OverSampleType::N2x);

        // Set output gain to compensate for level changes (reduce volume)
        output_gain.gain().set_value(0.3 / (1.0 + params.amount));

        // Connect the chain: input_gain -> wave_shaper -> output_gain
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

// Trait to unify different effect types
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
