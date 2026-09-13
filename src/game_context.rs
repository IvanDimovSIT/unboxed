use std::collections::HashSet;

use crate::{
    level::Level,
    level_context::LevelContext,
    resource_manager::ResourceManager,
    service::persistence::save_completed_levels,
    ui::{draw_help::draw_help, draw_level_select::draw_level_select},
};

#[derive(Debug)]
enum Mode {
    InLevel(LevelContext),
    LevelSelect,
    Help,
}

#[derive(Debug)]
pub enum Event {
    None,
    ChangeLevel(usize),
    ToLevelSelect,
    WinLevel(usize),
    ToHelp,
}

#[derive(Debug)]
pub struct GameContext<'a> {
    resource_manager: &'a ResourceManager,
    level_templates: &'a [Level],
    completed_levels: HashSet<usize>,
    mode: Mode,
}
impl<'a> GameContext<'a> {
    pub fn new(
        resource_manager: &'a ResourceManager,
        levels: &'a [Level],
        completed_levels: HashSet<usize>,
    ) -> Self {
        Self {
            resource_manager,
            level_templates: levels,
            mode: Mode::LevelSelect,
            completed_levels,
        }
    }

    pub fn process_frame(&mut self, delta: f32) {
        let event = match &mut self.mode {
            Mode::InLevel(ctx) => {
                ctx.process_level(self.level_templates.len(), self.resource_manager, delta)
            }
            Mode::LevelSelect => draw_level_select(
                self.level_templates.len(),
                &self.completed_levels,
                self.resource_manager,
            ),
            Mode::Help => draw_help(self.resource_manager),
        };

        match event {
            Event::None => {}
            Event::ChangeLevel(new_level) => {
                if new_level < self.level_templates.len() {
                    self.mode = Mode::InLevel(LevelContext::new(
                        self.level_templates[new_level].clone(),
                        new_level,
                    ))
                }
            }
            Event::ToLevelSelect => self.mode = Mode::LevelSelect,
            Event::ToHelp => self.mode = Mode::Help,
            Event::WinLevel(won_level) => {
                self.completed_levels.insert(won_level);
                save_completed_levels(&self.completed_levels)
            }
        }
    }
}
