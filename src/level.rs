use crate::{
    game_context::Event,
    graphics::{
        background::draw_background,
        draw::{LevelDrawContext, draw_level},
        level_window::find_level_window_position,
    },
    input,
    resource_manager::{ResourceManager, SoundId},
    service::{self, movement::MovementDelta},
    ui::{buttons::draw_back_button, message::display_message},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum AboveTile {
    #[default]
    None,
    Player,
    Box,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FloorTile {
    #[default]
    None,
    BoxExit,
    PlayerExit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    floor_tiles: [FloorTile; Self::TOTAL_TILES],
    above_tiles: [AboveTile; Self::TOTAL_TILES],
}
impl Level {
    pub const LEVEL_WIDTH: usize = 18;
    pub const LEVEL_HEIGHT: usize = 14;
    const TOTAL_TILES: usize = Self::LEVEL_WIDTH * Self::LEVEL_HEIGHT;

    pub fn get_above(&self, x: i32, y: i32) -> AboveTile {
        self.above_tiles[Self::index(x, y)]
    }

    pub fn get_below(&self, x: i32, y: i32) -> FloorTile {
        self.floor_tiles[Self::index(x, y)]
    }

    pub fn set_above(&mut self, tile: AboveTile, x: i32, y: i32) {
        self.above_tiles[Self::index(x, y)] = tile;
    }

    pub fn set_below(&mut self, tile: FloorTile, x: i32, y: i32) {
        self.floor_tiles[Self::index(x, y)] = tile;
    }

    fn index(x: i32, y: i32) -> usize {
        assert!(x >= 0 && x < Self::LEVEL_WIDTH as i32);
        assert!(y >= 0 && y < Self::LEVEL_HEIGHT as i32);
        x as usize + y as usize * Self::LEVEL_WIDTH
    }
}
impl Default for Level {
    fn default() -> Self {
        Self {
            floor_tiles: [Default::default(); Self::TOTAL_TILES],
            above_tiles: [Default::default(); Self::TOTAL_TILES],
        }
    }
}

#[derive(Debug, Clone)]
pub struct LevelContext {
    level_template: Level,
    pub level: Level,
    pub previous_deltas: Vec<Vec<MovementDelta>>,
    pub animation_time_s: f32,
    pub animation_deltas: Vec<MovementDelta>,
    pub current_level_index: usize,
    pub is_win: bool,
    pub cached_move: Option<(i32, i32)>,
}
impl LevelContext {
    const PREVIOUS_DELTAS_CAPACITY: usize = 32;
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
        }
    }

    pub fn reset(&mut self) {
        self.level = self.level_template.clone();
        self.previous_deltas.clear();
        self.animation_deltas.clear();
        self.animation_time_s = 0.0;
        self.cached_move = None;
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

        let result = service::movement::process(self);
        match result {
            service::movement::ProcessResult::None => {}
            service::movement::ProcessResult::Movement(movement_deltas) => {
                Self::play_sounds_for_deltas(&movement_deltas, resource_manager);
                self.animation_deltas = movement_deltas.clone();
                self.animation_time_s = 0.0;
                self.previous_deltas.push(movement_deltas);
            }
            service::movement::ProcessResult::Undo(undo_movement_deltas) => {
                self.animation_deltas = undo_movement_deltas;
                self.animation_time_s = 0.0;
            }
            service::movement::ProcessResult::Reset => {
                self.animation_deltas = vec![];
                self.animation_time_s = 0.0;
            }
        }

        if !self.animation_deltas.is_empty() {
            self.animation_time_s += delta;
        }
        if self.animation_time_s >= Self::ANIMATION_TIME {
            self.animation_deltas = vec![];
            self.animation_time_s = 0.0;
        }

        let animation_progress = self.animation_time_s / Self::ANIMATION_TIME;
        let window_pos = find_level_window_position();
        draw_background(resource_manager);
        draw_level(LevelDrawContext {
            animation_progress,
            start_x: window_pos.start_x,
            start_y: window_pos.start_y,
            width: window_pos.width,
            level: &self.level,
            deltas: &self.animation_deltas,
            resource_manager,
        });

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
