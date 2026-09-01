use macroquad::{color::Color, prelude::error, texture::Image};

use crate::level::{AboveTile, FloorTile, Level};

const L1_F: &[u8] = include_bytes!("../../levels/1/floor.png");
const L1_T: &[u8] = include_bytes!("../../levels/1/tiles.png");

const L2_F: &[u8] = include_bytes!("../../levels/2/floor.png");
const L2_T: &[u8] = include_bytes!("../../levels/2/tiles.png");

const BOX_EXIT_COLOR: Color = Color::from_rgba(255, 0, 0, 255);
const BOX_COLOR: Color = Color::from_rgba(255, 0, 0, 255);
const WALL_COLOR: Color = Color::from_rgba(255, 255, 255, 255);
const PLAYER_COLOR: Color = Color::from_rgba(0, 255, 0, 255);
const EMPTY_TILE_COLOR: Color = Color::from_rgba(0, 0, 0, 255);

pub fn load_levels() -> Vec<Level> {
    let mut levels = vec![];
    levels.push(load_level(L1_F, L1_T));
    levels.push(load_level(L2_F, L2_T));

    levels
}

fn load_level(f_bytes: &[u8], t_bytes: &[u8]) -> Level {
    let floor_image =
        Image::from_file_with_format(f_bytes, None).expect("Failed to load floor tiles");
    let tiles_image = Image::from_file_with_format(t_bytes, None).expect("Failed to load tiles");
    if floor_image.width() != Level::LEVEL_WIDTH {
        error!("Invalid level width {}", floor_image.width());
    }
    if floor_image.height() != Level::LEVEL_HEIGHT {
        error!("Invalid level height {}", floor_image.height());
    }

    let mut level = Level::default();

    for y in 0..Level::LEVEL_HEIGHT as u32 {
        for x in 0..Level::LEVEL_WIDTH as u32 {
            let floor_color = floor_image.get_pixel(x, y);
            let tile_color = tiles_image.get_pixel(x, y);
            level.set_below(color_to_floor(floor_color), x as i32, y as i32);
            level.set_above(color_to_tile(tile_color), x as i32, y as i32);
        }
    }

    level
}

fn color_to_floor(color: Color) -> FloorTile {
    if color == BOX_EXIT_COLOR {
        FloorTile::BoxExit
    } else if color == EMPTY_TILE_COLOR {
        FloorTile::None
    } else {
        error!("Invalid floor level color detected: {:?}", color);
        FloorTile::None
    }
}

fn color_to_tile(color: Color) -> AboveTile {
    if color == BOX_COLOR {
        AboveTile::Box
    } else if color == WALL_COLOR {
        AboveTile::Wall
    } else if color == PLAYER_COLOR {
        AboveTile::Player
    } else if color == EMPTY_TILE_COLOR {
        AboveTile::None
    } else {
        error!("Invalid tile level color detected: {:?}", color);
        AboveTile::None
    }
}
