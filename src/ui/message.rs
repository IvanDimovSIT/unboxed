use macroquad::{
    color::Color,
    miniquad::window::screen_size,
    text::{TextParams, draw_text_ex, measure_text},
};

use crate::resource_manager::ResourceManager;

const TEXT_COLOR: Color = Color::from_rgba(255, 255, 255, 255);
const TEXT_SHADOW_COLOR: Color = Color::from_rgba(255, 255, 255, 70);

pub fn display_message(messages: &[&str], resource_manager: &ResourceManager) {
    const SIZE_COEF: f32 = 0.05;
    let (width, height) = screen_size();
    let text_size = (SIZE_COEF * width) as u16;
    let text_height =
        measure_text(messages[0], Some(&resource_manager.font), text_size, 1.0).height;
    let start_y = (height - text_height * messages.len() as f32) / 2.;

    for (line, message) in messages.iter().enumerate() {
        let text_dimensions = measure_text(message, Some(&resource_manager.font), text_size, 1.0);

        let x = (width - text_dimensions.width) / 2.0;
        let y = start_y + line as f32 * text_height;

        draw_text_ex(
            message,
            x + 3.0,
            y + 3.0,
            TextParams {
                font: Some(&resource_manager.font),
                font_size: text_size,
                font_scale: 1.0,
                color: TEXT_SHADOW_COLOR,
                ..Default::default()
            },
        );
        draw_text_ex(
            message,
            x,
            y,
            TextParams {
                font: Some(&resource_manager.font),
                font_size: text_size,
                font_scale: 1.0,
                color: TEXT_COLOR,
                ..Default::default()
            },
        );
    }
}

pub fn draw_centered_text(text: &str, y_coef: f32, size: f32, resource_manager: &ResourceManager) {
    let (width, height) = screen_size();
    let text_size = (size * width) as u16;

    let text_dimensions = measure_text(text, Some(&resource_manager.font), text_size, 1.0);

    let x = (width - text_dimensions.width) / 2.0;
    let y = y_coef * height + text_dimensions.height;

    draw_text_ex(
        text,
        x + 3.0,
        y + 3.0,
        TextParams {
            font: Some(&resource_manager.font),
            font_size: text_size,
            font_scale: 1.0,
            color: TEXT_SHADOW_COLOR,
            ..Default::default()
        },
    );
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(&resource_manager.font),
            font_size: text_size,
            font_scale: 1.0,
            color: TEXT_COLOR,
            ..Default::default()
        },
    );
}
