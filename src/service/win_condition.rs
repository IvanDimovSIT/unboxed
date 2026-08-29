use crate::level::{AboveTile, FloorTile, Level};

pub fn is_win(level: &Level) -> bool {
    for y in 0..Level::LEVEL_HEIGHT as i32 {
        for x in 0..Level::LEVEL_WIDTH as i32 {
            let floor = level.get_below(x, y);
            match floor {
                FloorTile::None => continue,
                FloorTile::BoxExit => {
                    let tile = level.get_above(x, y);
                    if tile != AboveTile::Box {
                        return false;
                    }
                }
            }
        }
    }

    true
}
