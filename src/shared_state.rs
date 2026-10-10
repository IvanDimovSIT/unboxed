use std::collections::HashSet;

use crate::{level::Level, resource_manager::ResourceManager};

#[derive(Debug)]
pub struct SharedState {
    pub resource_manager: ResourceManager,
    pub level_templates: Vec<Level>,
    pub completed_levels: HashSet<usize>,
    pub is_mobile: bool,
}
