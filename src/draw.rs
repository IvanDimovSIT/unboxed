use macroquad::{
    color::Color,
    math::{Vec2, vec2},
    shapes::draw_rectangle,
};

use crate::level::{AboveTile, FloorTile, Level};

#[derive(Debug, Clone, Copy)]
pub struct LevelDrawContext {
    start_x: f32,
    start_y: f32,
    size: f32,
}
impl Default for LevelDrawContext {
    fn default() -> Self {
        Self {
            start_x: 0.0,
            start_y: 0.0,
            size: 600.0,
        }
    }
}

pub fn draw_level(level: &Level, context: LevelDrawContext) {
    let tile_size = context.size / Level::LEVEL_SIZE as f32;

    for y in 0..Level::LEVEL_SIZE {
        for x in 0..Level::LEVEL_SIZE {
            let pos_x = x as f32 * tile_size + context.start_x;
            let pos_y = y as f32 * tile_size + context.start_y;
            let pos = vec2(pos_x, pos_y);
            let above_tile = level.get_above(x as i32, y as i32);

            if above_tile != AboveTile::None {
                draw_above_tile(above_tile, pos, tile_size);
            } else {
                let floor_tile = level.get_below(x as i32, y as i32);
                draw_floor_tile(floor_tile, pos, tile_size);
            }
        }
    }
}

fn draw_above_tile(tile: AboveTile, pos: Vec2, size: f32) {
    let color = match tile {
        AboveTile::None => Color::from_rgba(0, 0, 0, 0),
        AboveTile::Player => Color::from_rgba(40, 255, 70, 255),
        AboveTile::Box => Color::from_rgba(255, 255, 90, 255),
        AboveTile::Wall => Color::from_rgba(60, 60, 90, 255),
    };

    draw_rectangle(pos.x, pos.y, size, size, color);
}

fn draw_floor_tile(tile: FloorTile, pos: Vec2, size: f32) {
    let color = match tile {
        FloorTile::None => Color::from_rgba(30, 30, 40, 255),
        FloorTile::BoxExit => Color::from_rgba(220, 220, 40, 255),
    };

    draw_rectangle(pos.x, pos.y, size, size, color);
}
