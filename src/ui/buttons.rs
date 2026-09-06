use macroquad::{
    color::WHITE,
    input::mouse_position,
    math::{Rect, Vec2, vec2},
    miniquad::window::screen_size,
    text::{TextParams, draw_text_ex, measure_text},
    texture::{DrawTextureParams, draw_texture_ex},
};

use crate::{
    input,
    resource_manager::{ResourceManager, SoundId},
};

const TEXT_SIZE_COEF: f32 = 0.8;
const BTN_SIZE_COEF: f32 = 0.1;
const POS_COEF: f32 = 0.015;

pub struct DrawLevelButtonContext<'a> {
    pub size: f32,
    pub resource_manager: &'a ResourceManager,
    pub mouse_pos: Vec2,
}

pub fn draw_square_button(text: &str, x: f32, y: f32, context: &DrawLevelButtonContext) -> bool {
    let button_rect = Rect::new(x, y, context.size, context.size);
    let is_hovered = button_rect.contains(context.mouse_pos);
    let texture = if is_hovered {
        &context.resource_manager.level_button_selected
    } else {
        &context.resource_manager.level_button
    };
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::splat(context.size)),
            ..Default::default()
        },
    );
    let font_size = (context.size * TEXT_SIZE_COEF) as u16;

    let text_dimensions = measure_text(text, Some(&context.resource_manager.font), font_size, 1.0);
    let margin_x = text_dimensions.width * 0.2;
    let margin_y = text_dimensions.height * 1.2;

    draw_text_ex(
        text,
        x + margin_x,
        y + margin_y,
        TextParams {
            font: Some(&context.resource_manager.font),
            color: WHITE,
            font_size,
            font_scale: 1.0,
            ..Default::default()
        },
    );

    let is_pressed = is_hovered && input::click();
    if is_pressed {
        context.resource_manager.play_sound(SoundId::Button);
    }

    is_pressed
}

pub fn draw_back_button(resource_manager: &ResourceManager) -> bool {
    let (width, height) = screen_size();
    let (mouse_x, mouse_y) = mouse_position();
    let mouse_pos = vec2(mouse_x, mouse_y);
    let button_size = BTN_SIZE_COEF * height;
    let smaller_size = width.min(height);
    let xy = POS_COEF * smaller_size;

    let button_rect = Rect::new(xy, xy, button_size, button_size);
    let is_hovered = button_rect.contains(mouse_pos);
    let texture = if is_hovered {
        &resource_manager.back_button_selected
    } else {
        &resource_manager.back_button
    };
    draw_texture_ex(
        texture,
        xy,
        xy,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::splat(button_size)),
            ..Default::default()
        },
    );
    let is_pressed = is_hovered && input::click();
    if is_pressed {
        resource_manager.play_sound(SoundId::Button);
    }

    is_pressed
}

pub fn draw_help_button(resource_manager: &ResourceManager) -> bool {
    let (width, height) = screen_size();
    let (mouse_x, mouse_y) = mouse_position();
    let mouse_pos = vec2(mouse_x, mouse_y);
    let button_size = BTN_SIZE_COEF * height;
    let smaller_size = width.min(height);
    let xy = POS_COEF * smaller_size;

    let context = &DrawLevelButtonContext {
        size: button_size,
        resource_manager,
        mouse_pos,
    };

    draw_square_button(" ?", xy, xy, context)
}
