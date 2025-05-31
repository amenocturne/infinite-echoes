// ./game/src/nodes/audio_effect.rs
use web_sys::BiquadFilterType;

#[derive(Clone, PartialEq)]
pub enum AudioEffect {
    Filter(FilterParameters),
    Distortion(DistortionParameters),
}

#[derive(Clone, PartialEq)]
pub struct FilterParameters {
    pub filter_type: FilterType,
    pub frequency: f32, // Cutoff frequency in Hz
    pub q: f32,         // Quality factor
    pub gain: f32,      // For peaking/shelving filters (in dB)
}

#[derive(Clone, PartialEq)]
pub struct DistortionParameters {
    pub amount: f32,
    pub curve_type: DistortionCurve,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    LowShelf,
    HighShelf,
    Peaking,
    Notch,
    AllPass,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DistortionCurve {
    SoftClip,
    HardClip,
}

impl FilterType {
    pub fn to_biquad_type(&self) -> BiquadFilterType {
        match self {
            FilterType::LowPass => BiquadFilterType::Lowpass,
            FilterType::HighPass => BiquadFilterType::Highpass,
            FilterType::BandPass => BiquadFilterType::Bandpass,
            FilterType::LowShelf => BiquadFilterType::Lowshelf,
            FilterType::HighShelf => BiquadFilterType::Highshelf,
            FilterType::Peaking => BiquadFilterType::Peaking,
            FilterType::Notch => BiquadFilterType::Notch,
            FilterType::AllPass => BiquadFilterType::Allpass,
        }
    }
}

impl AudioEffect {
    // Constructor methods for each effect type
    pub fn new_filter(filter_type: FilterType, frequency: f32, q: f32, gain: f32) -> Self {
        AudioEffect::Filter(FilterParameters {
            filter_type,
            frequency,
            q,
            gain,
        })
    }

    pub fn new_distortion(amount: f32, curve_type: DistortionCurve) -> Self {
        AudioEffect::Distortion(DistortionParameters { amount, curve_type })
    }

    // Default constructors
    pub fn default_filter() -> Self {
        Self::new_filter(FilterType::LowPass, 1000.0, 1.0, 0.0)
    }

    pub fn default_distortion() -> Self {
        // Changed default to SoftClip for a less harsh effect
        Self::new_distortion(0.5, DistortionCurve::SoftClip)
    }

    // Helper methods to check effect type
    pub fn is_filter(&self) -> bool {
        matches!(self, AudioEffect::Filter(_))
    }

    pub fn is_distortion(&self) -> bool {
        matches!(self, AudioEffect::Distortion(_))
    }

    // Getter methods with pattern matching
    pub fn as_filter(&self) -> Option<&FilterParameters> {
        match self {
            AudioEffect::Filter(params) => Some(params),
            _ => None,
        }
    }

    pub fn as_distortion(&self) -> Option<&DistortionParameters> {
        match self {
            AudioEffect::Distortion(params) => Some(params),
            _ => None,
        }
    }
}
