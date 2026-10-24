use std::borrow::Cow;

use macroquad::math::{Vec2, vec2};

use crate::{
    graphics::{shader::Light, tile_renderer::TileRenderer},
    ivec2::Ivec2,
    level::{AboveTile, Level},
    resource_manager::ResourceManager,
    service::movement::Action,
};

#[derive(Debug, Clone, Copy)]
pub struct LevelDrawContext<'a> {
    pub top_left: Vec2,
    pub width: f32,
    /// values: [0.0 - 1.0]
    pub animation_progress: f32,
    pub level: &'a Level,
    pub actions: &'a [Action],
    pub resource_manager: &'a ResourceManager,
}

pub fn draw_level(context: &LevelDrawContext) -> Vec<Light> {
    let tile_size = calculate_tile_size(context);
    let tile_center_offset = calculate_tile_center_offset(tile_size);
    let mut tile_renderer = TileRenderer::new();

    let level_to_draw = preapare_level_to_draw(context);
    let mut lights = Vec::with_capacity(50);

    for y in 0..Level::LEVEL_HEIGHT {
        for x in 0..Level::LEVEL_WIDTH {
            let pos = vec2(x as f32, y as f32) * tile_size + context.top_left;
            let above_tile = level_to_draw.get_above(x as i32, y as i32);

            if above_tile != AboveTile::None {
                if above_tile != AboveTile::Wall {
                    let floor_tile = level_to_draw.get_below(x as i32, y as i32);
                    tile_renderer.prepare_floor(floor_tile, pos);
                    if let Some(light) = Light::from_floor(floor_tile, pos + tile_center_offset) {
                        lights.push(light)
                    }
                }
                tile_renderer.prepare_tile(above_tile, pos);
                if let Some(light) = Light::from_tile(above_tile, pos + tile_center_offset) {
                    lights.push(light)
                }
            } else {
                let floor_tile = level_to_draw.get_below(x as i32, y as i32);
                tile_renderer.prepare_floor(floor_tile, pos);
                if let Some(light) = Light::from_floor(floor_tile, pos + tile_center_offset) {
                    lights.push(light)
                }
            }
        }
    }

    if !context.actions.is_empty() {
        draw_animated_tiles(context, &mut tile_renderer, &mut lights)
    };

    tile_renderer.render(tile_size, context.resource_manager);

    lights
}

fn preapare_level_to_draw<'a>(context: &'a LevelDrawContext) -> Cow<'a, Level> {
    if context.actions.is_empty() {
        Cow::Borrowed(context.level)
    } else {
        Cow::Owned(create_level_to_draw(context.level, context.actions))
    }
}

fn draw_animated_tiles(
    context: &LevelDrawContext,
    tile_renderer: &mut TileRenderer,
    lights: &mut Vec<Light>,
) {
    const EXPLOSION_ANIMATION: f32 = 0.5;
    assert!(context.animation_progress >= 0.0);
    assert!(context.animation_progress <= 1.0);

    let tile_size = calculate_tile_size(context);
    let tile_center_offset = calculate_tile_center_offset(tile_size);
    let coef = context.animation_progress;
    let r_coef = 1.0 - context.animation_progress;
    for action in context.actions {
        match *action {
            Action::Move(movement) => {
                let from: Vec2 = movement.from.into();
                let to: Vec2 = movement.to.into();

                let pos = (from * r_coef + (to * coef)) * tile_size + context.top_left;

                tile_renderer.prepare_tile(movement.tile, pos);
                if let Some(light) = Light::from_tile(movement.tile, pos + tile_center_offset) {
                    lights.push(light)
                }
            }
            Action::Explosion(ivec2) => {
                if coef > EXPLOSION_ANIMATION {
                    add_explosion(ivec2, tile_renderer, lights, tile_size, context.top_left);
                }
            }
            Action::UndoExplosion(ivec2) => {
                if coef <= EXPLOSION_ANIMATION {
                    add_explosion(ivec2, tile_renderer, lights, tile_size, context.top_left);
                }
            }
        }
    }
}

fn add_explosion(
    explosion_location: Ivec2,
    tile_renderer: &mut TileRenderer,
    lights: &mut Vec<Light>,
    tile_size: f32,
    top_left: Vec2,
) {
    let explostion_pos: Vec2 = explosion_location.into();
    let screen_explostion_pos: Vec2 = explostion_pos * tile_size + top_left;
    tile_renderer.prepare_explosion(screen_explostion_pos);

    let explosion_center = screen_explostion_pos + calculate_tile_center_offset(tile_size);
    lights.push(Light::explosion(explosion_center));
}

fn calculate_tile_center_offset(tile_size: f32) -> Vec2 {
    Vec2::splat(tile_size / 2.0)
}

fn calculate_tile_size(context: &LevelDrawContext) -> f32 {
    context.width / Level::LEVEL_WIDTH as f32
}

fn create_level_to_draw(level: &Level, actions: &[Action]) -> Level {
    let mut new_level = level.clone();
    for action in actions {
        if let Action::Move(movement) = *action {
            new_level.set_above(AboveTile::None, movement.to.x, movement.to.y);
        }
    }

    new_level
}
