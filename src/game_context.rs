use macroquad::{color::BLACK, window::clear_background};

use crate::{
    graphics::{
        draw::{LevelDrawContext, draw_level},
        level_window::find_level_window_position,
    },
    input,
    level::{Level, LevelContext},
    service::{self, movement::MovementDelta},
};

#[derive(Debug)]
enum Mode {
    InLevel(LevelContext),
    InMenu,
}

#[derive(Debug)]
enum Event {
    None,
    ChangeLevel(usize),
}

#[derive(Debug)]
pub struct GameContext<'a> {
    level_templates: &'a [Level],
    mode: Mode,
}
impl<'a> GameContext<'a> {
    const ANIMATION_TIME: f32 = 0.1;

    pub fn new(levels: &'a [Level]) -> Self {
        Self {
            level_templates: levels,
            mode: Mode::InLevel(LevelContext::new(levels[0].clone(), 0)),
        }
    }

    pub fn process_frame(&mut self, delta: f32) {
        clear_background(BLACK);
        let event = match &mut self.mode {
            Mode::InLevel(ctx) => Self::process_level(ctx, delta),
            Mode::InMenu => todo!(),
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
        }
    }

    fn process_level(level_context: &mut LevelContext, delta: f32) -> Event {
        let result = service::movement::process(level_context);
        match result {
            service::movement::ProcessResult::None => {}
            service::movement::ProcessResult::Movement(movement_deltas) => {
                level_context.animation_deltas = movement_deltas.clone();
                level_context.animation_time_s = 0.0;
                level_context.previous_deltas.push(movement_deltas);
            }
            service::movement::ProcessResult::Undo(undo_movement_deltas) => {
                level_context.animation_deltas = undo_movement_deltas;
                level_context.animation_time_s = 0.0;
            }
            service::movement::ProcessResult::Reset => {
                level_context.animation_deltas = vec![];
                level_context.animation_time_s = 0.0;
            }
        }

        level_context.animation_time_s += delta;
        if level_context.animation_time_s >= Self::ANIMATION_TIME {
            level_context.animation_deltas = vec![];
            level_context.animation_time_s = 0.0;
        }

        let animation_progress = level_context.animation_time_s / Self::ANIMATION_TIME;
        let window_pos = find_level_window_position();
        draw_level(
            &level_context.level,
            &level_context.animation_deltas,
            LevelDrawContext {
                animation_progress,
                start_x: window_pos.start_x,
                start_y: window_pos.start_y,
                width: window_pos.width,
            },
        );

        if !level_context.is_win && service::win_condition::is_win(&level_context.level) {
            level_context.is_win = true;
            Event::None
        } else if level_context.is_win && input::any_input() {
            Event::ChangeLevel(level_context.current_level_index + 1)
        } else {
            Event::None
        }
    }
}
