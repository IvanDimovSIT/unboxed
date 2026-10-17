use std::collections::HashSet;

use crate::{
    level::Level,
    resource_manager::{ResourceManager, SoundId},
};

#[derive(Debug)]
pub struct SharedState {
    pub resource_manager: ResourceManager,
    pub level_templates: Vec<Level>,
    pub completed_levels: HashSet<usize>,
    pub is_mobile: bool,
    pub is_sound_on: bool,
}
impl SharedState {
    pub fn play_sound_if_enabled(&self, sound_id: SoundId) {
        if self.is_sound_on {
            self.resource_manager.play_sound(sound_id);
        }
    }
}
