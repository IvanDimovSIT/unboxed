use macroquad::{input::mouse_position, math::vec2, miniquad::window::screen_size};

use crate::{
    graphics::{
        background::draw_background,
        draw::{LevelDrawContext, draw_level},
        level_window::find_level_window_position,
    },
    input,
    level::{AboveTile, Level, LevelContext},
    resource_manager::{ResourceManager, SoundId},
    service::{self, movement::MovementDelta},
    ui::{
        buttons::{DrawLevelButtonContext, draw_back_button},
        draw_level_select::draw_level_select,
        message::display_message,
    },
};

#[derive(Debug)]
enum Mode {
    InLevel(LevelContext),
    InMenu,
}

#[derive(Debug)]
pub enum Event {
    None,
    ChangeLevel(usize),
    ToMenu,
}

#[derive(Debug)]
pub struct GameContext<'a> {
    resource_manager: &'a ResourceManager,
    level_templates: &'a [Level],
    mode: Mode,
}
impl<'a> GameContext<'a> {
    const ANIMATION_TIME: f32 = 0.1;

    pub fn new(resource_manager: &'a ResourceManager, levels: &'a [Level]) -> Self {
        Self {
            resource_manager,
            level_templates: levels,
            mode: Mode::InMenu,
        }
    }

    pub fn process_frame(&mut self, delta: f32) {
        let event = match &mut self.mode {
            Mode::InLevel(ctx) => Self::process_level(
                ctx,
                self.level_templates.len(),
                self.resource_manager,
                delta,
            ),
            Mode::InMenu => draw_level_select(self.level_templates.len(), self.resource_manager),
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
            Event::ToMenu => self.mode = Mode::InMenu,
        }
    }

    fn process_level(
        level_context: &mut LevelContext,
        levels_count: usize,
        resource_manager: &'a ResourceManager,
        delta: f32,
    ) -> Event {
        if input::exit() {
            return Event::ToMenu;
        }

        let result = service::movement::process(level_context);
        match result {
            service::movement::ProcessResult::None => {}
            service::movement::ProcessResult::Movement(movement_deltas) => {
                Self::play_sounds_for_deltas(&movement_deltas, resource_manager);
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

        if !level_context.animation_deltas.is_empty() {
            level_context.animation_time_s += delta;
        }
        if level_context.animation_time_s >= Self::ANIMATION_TIME {
            level_context.animation_deltas = vec![];
            level_context.animation_time_s = 0.0;
        }

        let animation_progress = level_context.animation_time_s / Self::ANIMATION_TIME;
        let window_pos = find_level_window_position();
        draw_background(resource_manager);
        draw_level(LevelDrawContext {
            animation_progress,
            start_x: window_pos.start_x,
            start_y: window_pos.start_y,
            width: window_pos.width,
            level: &level_context.level,
            deltas: &level_context.animation_deltas,
            resource_manager,
        });

        if level_context.is_win {
            display_message(
                &["Level complete!", "Press any key to continue..."],
                resource_manager,
            );
        }

        if Self::draw_back_button(resource_manager) {
            return Event::ToMenu;
        }

        if !level_context.is_win && service::win_condition::is_win(&level_context.level) {
            level_context.is_win = true;
            resource_manager.play_sound(SoundId::Win);
            Event::None
        } else if level_context.is_win && input::any_input() {
            if level_context.current_level_index + 1 == levels_count {
                Event::ToMenu
            } else {
                Event::ChangeLevel(level_context.current_level_index + 1)
            }
        } else {
            Event::None
        }
    }

    fn draw_back_button(resource_manager: &ResourceManager) -> bool {
        const BTN_SIZE_COEF: f32 = 0.07;
        const POS_COEF: f32 = 0.02;
        let (width, height) = screen_size();
        let (mouse_x, mouse_y) = mouse_position();
        let button_suze = BTN_SIZE_COEF * height;
        let ctx = DrawLevelButtonContext {
            size: button_suze,
            resource_manager,
            mouse_pos: vec2(mouse_x, mouse_y),
        };

        let smaller_size = width.min(height);
        let xy = POS_COEF * smaller_size;
        draw_back_button(xy, xy, &ctx)
    }

    fn play_sounds_for_deltas(deltas: &[MovementDelta], resource_manager: &ResourceManager) {
        for d in deltas {
            if d.tile == AboveTile::Box {
                resource_manager.play_sound(SoundId::PushBox);
                break;
            }
        }
        for d in deltas {
            if d.tile == AboveTile::Player {
                resource_manager.play_sound(SoundId::Move);
                return;
            }
        }
    }
}
