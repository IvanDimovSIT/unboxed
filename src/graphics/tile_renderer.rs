use macroquad::{
    color::WHITE,
    math::Vec2,
    texture::{DrawTextureParams, draw_texture_ex},
};

use crate::{
    level::{AboveTile, FloorTile, Level},
    resource_manager::ResourceManager,
};

#[derive(Debug)]
pub struct TileRenderer {
    above_tiles: Vec<AboveTileDrawCommand>,
    floor_tiles: Vec<FloorTileDrawCommand>,
}
impl TileRenderer {
    pub fn new() -> Self {
        let above_tiles = Vec::with_capacity(Level::TOTAL_TILES);
        let floor_tiles = Vec::with_capacity(Level::TOTAL_TILES);
        Self {
            above_tiles,
            floor_tiles,
        }
    }

    pub fn prepare_tile(&mut self, tile: AboveTile, pos: Vec2) {
        if tile == AboveTile::None {
            return;
        }

        self.above_tiles.push(AboveTileDrawCommand { pos, tile });
    }

    pub fn prepare_floor(&mut self, tile: FloorTile, pos: Vec2) {
        self.floor_tiles.push(FloorTileDrawCommand { pos, tile });
    }

    pub fn render(&mut self, tile_size: f32, resource_manager: &ResourceManager) {
        self.above_tiles.sort_unstable_by_key(|dc| dc.tile);
        self.floor_tiles.sort_unstable_by_key(|dc| dc.tile);
        let dest_size = Some(Vec2::splat(tile_size));

        for draw_command in &self.floor_tiles {
            let texture = resource_manager.get_texture_for_floor(draw_command.tile);
            draw_texture_ex(
                texture,
                draw_command.pos.x,
                draw_command.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size,
                    ..Default::default()
                },
            );
        }
        for draw_command in &self.above_tiles {
            let texture = resource_manager.get_texture_for_tile(draw_command.tile);
            draw_texture_ex(
                texture,
                draw_command.pos.x,
                draw_command.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size,
                    ..Default::default()
                },
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct AboveTileDrawCommand {
    pos: Vec2,
    tile: AboveTile,
}

#[derive(Debug, Clone, Copy)]
struct FloorTileDrawCommand {
    pos: Vec2,
    tile: FloorTile,
}
