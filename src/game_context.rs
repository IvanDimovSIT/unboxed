use crate::{
    level_context::LevelContext,
    service::persistence::save_completed_levels,
    shared_state::SharedState,
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
pub struct GameContext {
    shared_state: SharedState,
    mode: Mode,
}
impl GameContext {
    pub fn new(shared_state: SharedState) -> Self {
        Self {
            shared_state,
            mode: Mode::LevelSelect,
        }
    }

    pub fn process_frame(&mut self, delta: f32) {
        let event = match &mut self.mode {
            Mode::InLevel(ctx) => ctx.process_level(&self.shared_state, delta),
            Mode::LevelSelect => draw_level_select(&self.shared_state),
            Mode::Help => draw_help(&self.shared_state),
        };

        match event {
            Event::None => {}
            Event::ChangeLevel(new_level) => {
                let show_controls = self.shared_state.completed_levels.is_empty();
                if new_level < self.shared_state.level_templates.len() {
                    self.mode = Mode::InLevel(LevelContext::new(
                        self.shared_state.level_templates[new_level].clone(),
                        new_level,
                        show_controls,
                    ))
                }
            }
            Event::ToLevelSelect => self.mode = Mode::LevelSelect,
            Event::ToHelp => self.mode = Mode::Help,
            Event::WinLevel(won_level) => {
                self.shared_state.completed_levels.insert(won_level);
                save_completed_levels(&self.shared_state.completed_levels)
            }
        }
    }
}
