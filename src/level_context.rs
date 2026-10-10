#[cfg(debug_assertions)]
use macroquad::prelude::info;
use macroquad::{
    math::{Vec2, vec2},
    miniquad::window::screen_size,
};

use crate::{
    game_context::Event,
    graphics::{
        background::draw_background,
        draw::{LevelDrawContext, draw_level},
        level_window::find_level_window_position,
    },
    input::{self, MovementInput, get_mouse_vec},
    level::{AboveTile, Level},
    resource_manager::{ResourceManager, SoundId},
    service::{
        self,
        movement::{self, MovementDelta},
    },
    shared_state::SharedState,
    ui::{
        buttons::Button,
        message::{display_message, draw_level_number},
    },
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
    show_controls: bool,
}
impl LevelContext {
    const PREVIOUS_DELTAS_CAPACITY: usize = 64;
    const ANIMATION_TIME: f32 = 0.1;

    pub fn new(level: Level, index: usize, show_controls: bool) -> Self {
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
            show_controls,
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

    pub fn process_level(&mut self, shared_state: &SharedState, delta: f32) -> Event {
        if input::exit() {
            return Event::ToLevelSelect;
        }

        let (width, height) = screen_size();
        let mouse_vec = get_mouse_vec();
        let mut back_button = Button::new_back(width, height);
        let mut undo_button = Button::new_undo(width, height);
        let mut reset_button = Button::new_reset(width, height);
        back_button.detect(mouse_vec, &shared_state.resource_manager);
        if shared_state.is_mobile {
            undo_button.detect(mouse_vec, &shared_state.resource_manager);
            reset_button.detect(mouse_vec, &shared_state.resource_manager);
        }

        if back_button.is_clicked {
            return Event::ToLevelSelect;
        }

        let movement_input = if undo_button.is_clicked {
            Some(MovementInput::Undo)
        } else if reset_button.is_clicked {
            Some(MovementInput::Reset)
        } else {
            self.get_movement(mouse_vec, width, height, shared_state)
        };
        if let Some(movement) = movement_input {
            self.show_controls = false;
            if movement == MovementInput::Undo {
                self.undo();
            } else if movement == MovementInput::Reset {
                self.reset();
            } else {
                self.handle_move(movement, &shared_state.resource_manager);
            }
        }

        if !self.animation_deltas.is_empty() {
            self.animation_time_s += delta;
        }
        if self.animation_time_s >= Self::ANIMATION_TIME {
            self.animation_deltas.clear();
            self.animation_time_s = 0.0;
        }

        self.draw_level(
            width,
            height,
            &back_button,
            &undo_button,
            &reset_button,
            shared_state,
        );

        if !self.is_win && service::win_condition::is_win(&self.level) {
            self.is_win = true;
            shared_state.resource_manager.play_sound(SoundId::Win);
            #[cfg(debug_assertions)]
            info!(
                "Winning moves: {}",
                MovementInput::slice_to_string(&self.inputs)
            );
            Event::WinLevel(self.current_level_index)
        } else if self.is_win && input::next_level(shared_state.is_mobile) {
            if self.current_level_index + 1 == shared_state.level_templates.len() {
                Event::ToLevelSelect
            } else {
                Event::ChangeLevel(self.current_level_index + 1)
            }
        } else {
            Event::None
        }
    }

    fn show_messages(&self, shared_state: &SharedState) {
        const LEVEL_COMPLETE_MESSAGES_DESKTOP: [&str; 2] =
            ["Level complete!", "Press space to continue..."];
        const LEVEL_COMPLETE_MESSAGES_MOBILE: [&str; 2] = ["Level complete!", "Tap to continue..."];
        const CONTROLS_MESSAGES_DESKTOP: [&str; 3] = ["W/S/A/D - Move", "Z - Undo", "R - Reset"];
        const CONTROLS_MESSAGES_MOBILE: [&str; 3] = [
            "Tap the sides of the screen to move",
            "Undo the last move with",
            "the button on the top right",
        ];
        let resource_manager = &shared_state.resource_manager;

        let messages = if self.is_win {
            if shared_state.is_mobile {
                LEVEL_COMPLETE_MESSAGES_MOBILE.as_slice()
            } else {
                LEVEL_COMPLETE_MESSAGES_DESKTOP.as_slice()
            }
        } else if self.show_controls {
            if shared_state.is_mobile {
                CONTROLS_MESSAGES_MOBILE.as_slice()
            } else {
                CONTROLS_MESSAGES_DESKTOP.as_slice()
            }
        } else {
            return;
        };

        display_message(messages, resource_manager);
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

    fn get_movement(
        &mut self,
        mouse_vec: Vec2,
        width: f32,
        height: f32,
        shared_state: &SharedState,
    ) -> Option<MovementInput> {
        if self.is_win {
            self.cached_move = None;
            return None;
        }

        let movement_input = if shared_state.is_mobile {
            input::get_movement_input_mobile(mouse_vec, width, height)
        } else {
            input::get_movement_input_desktop()
        };
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

    fn draw_level(
        &self,
        width: f32,
        height: f32,
        back_button: &Button,
        undo_button: &Button,
        reset_button: &Button,
        shared_state: &SharedState,
    ) {
        let resource_manager = &shared_state.resource_manager;
        let animation_progress = self.animation_time_s / Self::ANIMATION_TIME;
        let window_pos = find_level_window_position();
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
            let lights = draw_level(&level_draw_context);
            self.show_messages(shared_state);
            draw_level_number(self.current_level_index + 1, height, resource_manager);
            back_button.draw(resource_manager);
            if shared_state.is_mobile {
                undo_button.draw(resource_manager);
                reset_button.draw(resource_manager);
            }

            lights
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
