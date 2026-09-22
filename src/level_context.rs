#[cfg(debug_assertions)]
use macroquad::prelude::info;
use macroquad::{math::vec2, miniquad::window::screen_size};

use crate::{
    game_context::Event,
    graphics::{
        background::draw_background,
        draw::{LevelDrawContext, draw_level},
        level_window::find_level_window_position,
    },
    input::{self, MovementInput},
    level::{AboveTile, Level},
    resource_manager::{ResourceManager, SoundId},
    service::{
        self,
        movement::{self, MovementDelta},
    },
    ui::{buttons::draw_back_button, message::display_message},
};

#[derive(Debug, Clone)]
pub struct LevelContext {
    level_template: Level,
    cached_move: Option<MovementInput>,
    #[cfg(debug_assertions)]
    inputs: Vec<MovementInput>,
    level: Level,
    previous_deltas: Vec<Vec<MovementDelta>>,
    animation_time_s: f32,
    animation_deltas: Vec<MovementDelta>,
    current_level_index: usize,
    is_win: bool,
}
impl LevelContext {
    const PREVIOUS_DELTAS_CAPACITY: usize = 64;
    const ANIMATION_TIME: f32 = 0.1;

    pub fn new(level: Level, index: usize) -> Self {
        Self {
            level_template: level.clone(),
            level,
            previous_deltas: Vec::with_capacity(Self::PREVIOUS_DELTAS_CAPACITY),
            animation_time_s: 0.0,
            animation_deltas: vec![],
            current_level_index: index,
            is_win: false,
            cached_move: None,
            #[cfg(debug_assertions)]
            inputs: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.level = self.level_template.clone();
        self.previous_deltas.clear();
        self.animation_deltas.clear();
        self.animation_time_s = 0.0;
        self.cached_move = None;
        #[cfg(debug_assertions)]
        self.inputs.clear();
    }

    pub fn process_level(
        &mut self,
        levels_count: usize,
        resource_manager: &ResourceManager,
        delta: f32,
    ) -> Event {
        if input::exit() {
            return Event::ToLevelSelect;
        }

        let movement_input = self.get_movement();
        if let Some(movement) = movement_input {
            if movement == MovementInput::Undo {
                self.undo();
            } else if movement == MovementInput::Reset {
                self.reset();
            } else {
                self.handle_move(movement, resource_manager);
            }
        }

        if !self.animation_deltas.is_empty() {
            self.animation_time_s += delta;
        }
        if self.animation_time_s >= Self::ANIMATION_TIME {
            self.animation_deltas = vec![];
            self.animation_time_s = 0.0;
        }

        self.draw_level(resource_manager);

        if self.is_win {
            display_message(
                &["Level complete!", "Press space to continue..."],
                resource_manager,
            );
        }

        if draw_back_button(resource_manager) {
            return Event::ToLevelSelect;
        }

        if !self.is_win && service::win_condition::is_win(&self.level) {
            self.is_win = true;
            resource_manager.play_sound(SoundId::Win);
            #[cfg(debug_assertions)]
            info!(
                "Winning moves: {}",
                MovementInput::slice_to_string(&self.inputs)
            );
            Event::WinLevel(self.current_level_index)
        } else if self.is_win && input::next_level() {
            if self.current_level_index + 1 == levels_count {
                Event::ToLevelSelect
            } else {
                Event::ChangeLevel(self.current_level_index + 1)
            }
        } else {
            Event::None
        }
    }

    fn undo(&mut self) {
        if let Some(prev_deltas) = self.previous_deltas.pop() {
            let undo_deltas = movement::undo_deltas(&mut self.level, prev_deltas);
            self.animation_deltas = undo_deltas;
            self.animation_time_s = 0.0;
            self.cached_move = None;
            #[cfg(debug_assertions)]
            self.inputs.pop();
        }
    }

    fn handle_move(&mut self, movement: MovementInput, resource_manager: &ResourceManager) {
        assert!(matches!(
            movement,
            MovementInput::Left | MovementInput::Right | MovementInput::Up | MovementInput::Down
        ));
        let movement_deltas = service::movement::process(&mut self.level, movement);
        if !movement_deltas.is_empty() {
            #[cfg(debug_assertions)]
            self.inputs.push(movement);
            Self::play_sounds_for_deltas(&movement_deltas, resource_manager);
            self.animation_deltas = movement_deltas.clone();
            self.animation_time_s = 0.0;
            self.previous_deltas.push(movement_deltas);
        }
    }

    fn get_movement(&mut self) -> Option<MovementInput> {
        if self.is_win {
            self.cached_move = None;
            return None;
        }

        let movement_input = input::get_movement_input();
        if let Some(movement) = movement_input {
            if self.animation_deltas.is_empty() {
                movement_input
            } else if movement == MovementInput::Undo || movement == MovementInput::Reset {
                self.cached_move = None;
                movement_input
            } else {
                self.cached_move = movement_input;
                None
            }
        } else {
            let cached_move = self.cached_move;
            self.cached_move = None;
            cached_move
        }
    }

    fn draw_level(&self, resource_manager: &ResourceManager) {
        let animation_progress = self.animation_time_s / Self::ANIMATION_TIME;
        let window_pos = find_level_window_position();
        let (width, height) = screen_size();
        resource_manager.shader.use_shader(width, height, || {
            draw_background(resource_manager);
            let level_draw_context = LevelDrawContext {
                animation_progress,
                top_left: vec2(window_pos.start_x, window_pos.start_y),
                width: window_pos.width,
                level: &self.level,
                deltas: &self.animation_deltas,
                resource_manager,
            };

            draw_level(&level_draw_context)
        });
    }

    fn play_sounds_for_deltas(deltas: &[MovementDelta], resource_manager: &ResourceManager) {
        for d in deltas {
            if d.tile.is_box() {
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
