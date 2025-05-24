use std::cell::RefCell;

use super::audio_node::{AudioNode, DisplayedAudioNode};

pub struct AudioEffect {
    displayed_audio_node: RefCell<DisplayedAudioNode>,
}

impl AudioEffect {
    pub fn new(displayed_audio_node: DisplayedAudioNode) -> Self {
        Self {
            displayed_audio_node: RefCell::new(displayed_audio_node),
        }
    }
}

impl AudioNode for AudioEffect {
    fn as_displayed(&self) -> &RefCell<DisplayedAudioNode> {
        &self.displayed_audio_node
    }
}
