#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub enum AboveTile {
    #[default]
    None,
    Player,
    Box,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
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
    pub const TOTAL_TILES: usize = Self::LEVEL_WIDTH * Self::LEVEL_HEIGHT;

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
