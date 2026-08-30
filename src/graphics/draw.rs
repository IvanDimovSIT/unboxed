use std::borrow::Cow;

use macroquad::{
    color::WHITE,
    math::{Vec2, vec2},
    texture::{DrawTextureParams, draw_texture_ex},
};

use crate::{
    level::{AboveTile, FloorTile, Level},
    resource_manager::ResourceManager,
    service::movement::MovementDelta,
};

#[derive(Debug, Clone, Copy)]
pub struct LevelDrawContext<'a> {
    pub start_x: f32,
    pub start_y: f32,
    pub width: f32,
    /// values: [0.0 - 1.0]
    pub animation_progress: f32,
    pub level: &'a Level,
    pub deltas: &'a [MovementDelta],
    pub resource_manager: &'a ResourceManager,
}

pub fn draw_level(context: LevelDrawContext) {
    let tile_size = calculate_tile_size(context);
    let level_to_draw = if context.deltas.is_empty() {
        Cow::Borrowed(context.level)
    } else {
        Cow::Owned(create_level_to_draw(context.level, context.deltas))
    };

    for y in 0..Level::LEVEL_HEIGHT {
        for x in 0..Level::LEVEL_WIDTH {
            let pos_x = x as f32 * tile_size + context.start_x;
            let pos_y = y as f32 * tile_size + context.start_y;
            let pos = vec2(pos_x, pos_y);
            let above_tile = level_to_draw.get_above(x as i32, y as i32);

            if above_tile != AboveTile::None {
                if above_tile != AboveTile::Wall {
                    let floor_tile = level_to_draw.get_below(x as i32, y as i32);
                    draw_floor_tile(floor_tile, pos, tile_size, context.resource_manager);
                }
                draw_above_tile(above_tile, pos, tile_size, context.resource_manager);
            } else {
                let floor_tile = level_to_draw.get_below(x as i32, y as i32);
                draw_floor_tile(floor_tile, pos, tile_size, context.resource_manager);
            }
        }
    }

    if !context.deltas.is_empty() {
        draw_animated_tiles(context);
    }
}

fn draw_animated_tiles(context: LevelDrawContext) {
    assert!(context.animation_progress >= 0.0);
    assert!(context.animation_progress <= 1.0);
    let tile_size = calculate_tile_size(context);
    let coef = context.animation_progress;
    let r_coef = 1.0 - context.animation_progress;
    for d in context.deltas {
        let (from_x, from_y) = d.from;
        let (to_x, to_y) = d.to;

        let pos_x = ((from_x as f32 * r_coef) + (to_x as f32 * coef)) * tile_size + context.start_x;
        let pos_y = ((from_y as f32 * r_coef) + (to_y as f32 * coef)) * tile_size + context.start_y;
        let pos = vec2(pos_x, pos_y);
        draw_above_tile(d.tile, pos, tile_size, context.resource_manager);
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

fn draw_above_tile(tile: AboveTile, pos: Vec2, size: f32, resource_manager: &ResourceManager) {
    if tile == AboveTile::None {
        return;
    }

    let texture = resource_manager.get_texture_for_tile(tile);

    draw_texture_ex(
        texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::splat(size)),
            ..Default::default()
        },
    );
}

fn draw_floor_tile(tile: FloorTile, pos: Vec2, size: f32, resource_manager: &ResourceManager) {
    let texture = resource_manager.get_texture_for_floor(tile);

    draw_texture_ex(
        texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::splat(size)),
            ..Default::default()
        },
    );
}
