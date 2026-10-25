use macroquad::{
    color::{BLACK, Color, WHITE},
    math::{Vec2, vec2},
    miniquad::window::screen_size,
    texture::{DrawTextureParams, draw_texture_ex},
    window::clear_background,
};

use crate::{
    graphics::inner_window::find_rotate_screen_prompt_position, resource_manager::ResourceManager,
};

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

pub fn draw_rotate_screen_prompt(resource_manager: &ResourceManager) {
    clear_background(BLACK);
    let prompt_pos = find_rotate_screen_prompt_position(resource_manager);
    draw_texture_ex(
        &resource_manager.rotate_device_prompt,
        prompt_pos.start_x,
        prompt_pos.start_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(prompt_pos.width, prompt_pos.height)),
            ..Default::default()
        },
    );
}
