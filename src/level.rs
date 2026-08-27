use crate::service::movement::MovementDelta;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    floor_tiles: [FloorTile; Self::TOTAL_TILES],
    above_tiles: [AboveTile; Self::TOTAL_TILES],
}
impl Level {
    pub const LEVEL_WIDTH: usize = 16;
    pub const LEVEL_HEIGHT: usize = 16;
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
}
impl LevelContext {
    const PREVIOUS_DELTAS_CAPACITY: usize = 32;

    pub fn new(level: Level) -> Self {
        Self {
            level_template: level.clone(),
            level,
            previous_deltas: Vec::with_capacity(Self::PREVIOUS_DELTAS_CAPACITY),
        }
    }

    pub fn reset(&mut self) {
        self.level = self.level_template.clone();
        self.previous_deltas.clear();
    }
}
