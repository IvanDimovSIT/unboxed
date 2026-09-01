use macroquad::{
    color::Color,
    math::Vec2,
    miniquad::window::screen_size,
    texture::{DrawTextureParams, draw_texture_ex},
};

use crate::resource_manager::ResourceManager;

pub fn draw_background(resource_manager: &ResourceManager) {
    const BG_COLOR: Color = Color::from_rgba(60, 100, 100, 255);
    let (width, height) = screen_size();
    let size = width.max(height);
    let bg_texture = &resource_manager.background;

    draw_texture_ex(
        bg_texture,
        0.0,
        0.0,
        BG_COLOR,
        DrawTextureParams {
            dest_size: Some(Vec2::splat(size)),
            ..Default::default()
        },
    );
}
