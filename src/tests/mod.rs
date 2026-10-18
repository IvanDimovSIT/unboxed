use crate::level::{AboveTile, FloorTile, Level};

mod integration_tests;
mod movement_tests;
mod win_condition_tests;

const MAX_X: i32 = Level::LEVEL_WIDTH as i32 - 1;
const MAX_Y: i32 = Level::LEVEL_HEIGHT as i32 - 1;

fn create_level(floors: &[(FloorTile, i32, i32)], aboves: &[(AboveTile, i32, i32)]) -> Level {
    let mut level = Level::default();
    for &(tile, x, y) in floors {
        level.set_below(tile, x, y);
    }
    for &(tile, x, y) in aboves {
        level.set_above(tile, x, y);
    }
    level
}
