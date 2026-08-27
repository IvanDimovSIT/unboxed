use std::borrow::Cow;

use macroquad::{
    color::Color,
    math::{Vec2, vec2},
    shapes::draw_rectangle,
};

use crate::{
    level::{AboveTile, FloorTile, Level},
    service::movement::MovementDelta,
};

#[derive(Debug, Clone, Copy)]
pub struct LevelDrawContext {
    pub start_x: f32,
    pub start_y: f32,
    pub width: f32,
    /// values: [0.0 - 1.0]
    pub animation_progress: f32,
}
impl Default for LevelDrawContext {
    fn default() -> Self {
        Self {
            start_x: 0.0,
            start_y: 0.0,
            width: 600.0,
            animation_progress: 0.0,
        }
    }
}

pub fn draw_level(level: &Level, deltas: &[MovementDelta], context: LevelDrawContext) {
    let tile_size = calculate_tile_size(context);
    let level_to_draw = if deltas.is_empty() {
        Cow::Borrowed(level)
    } else {
        Cow::Owned(create_level_to_draw(level, deltas))
    };

    for y in 0..Level::LEVEL_HEIGHT {
        for x in 0..Level::LEVEL_WIDTH {
            let pos_x = x as f32 * tile_size + context.start_x;
            let pos_y = y as f32 * tile_size + context.start_y;
            let pos = vec2(pos_x, pos_y);
            let above_tile = level_to_draw.get_above(x as i32, y as i32);

            if above_tile != AboveTile::None {
                draw_above_tile(above_tile, pos, tile_size);
            } else {
                let floor_tile = level_to_draw.get_below(x as i32, y as i32);
                draw_floor_tile(floor_tile, pos, tile_size);
            }
        }
    }

    if !deltas.is_empty() {
        draw_animated_tiles(deltas, context);
    }
}

fn draw_animated_tiles(deltas: &[MovementDelta], context: LevelDrawContext) {
    assert!(context.animation_progress >= 0.0);
    assert!(context.animation_progress <= 1.0);
    let tile_size = calculate_tile_size(context);
    let coef = context.animation_progress;
    let r_coef = 1.0 - context.animation_progress;
    for d in deltas {
        let (from_x, from_y) = d.from;
        let (to_x, to_y) = d.to;

        let pos_x = ((from_x as f32 * r_coef) + (to_x as f32 * coef)) * tile_size + context.start_x;
        let pos_y = ((from_y as f32 * r_coef) + (to_y as f32 * coef)) * tile_size + context.start_y;
        let pos = vec2(pos_x, pos_y);
        draw_above_tile(d.tile, pos, tile_size);
    }
}

fn calculate_tile_size(context: LevelDrawContext) -> f32 {
    context.width / Level::LEVEL_WIDTH as f32
}

fn create_level_to_draw(level: &Level, deltas: &[MovementDelta]) -> Level {
    let mut new_level = level.clone();
    for d in deltas {
        let (x, y) = d.to;
        new_level.set_above(AboveTile::None, x, y);
    }

    new_level
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
