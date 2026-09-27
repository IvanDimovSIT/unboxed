use macroquad::{
    color::{BLACK, Color},
    miniquad::window::screen_size,
    shapes::draw_rectangle,
    text::{Font, TextDimensions, TextParams, draw_text_ex, measure_text},
    window::clear_background,
};

use crate::resource_manager::ResourceManager;

const TEXT_COLOR: Color = Color::from_rgba(255, 255, 255, 255);
const TEXT_SHADOW_COLOR: Color = Color::from_rgba(255, 255, 255, 70);

pub fn display_message(messages: &[&str], resource_manager: &ResourceManager) {
    const SIZE_COEF: f32 = 0.05;
    const BACKGROUND_COLOR: Color = Color::from_rgba(0, 0, 0, 100);
    let (screen_width, screen_height) = screen_size();
    if messages.is_empty() {
        return;
    }
    let text_size = (SIZE_COEF * screen_width) as u16;
    let mut text_dimensions_arr: [TextDimensions; 16] = Default::default();
    let mut max_width = 0.0f32;

    for (index, message) in messages.iter().enumerate() {
        let text_dimensions = measure_text(message, Some(&resource_manager.font), text_size, 1.0);
        text_dimensions_arr[index] = text_dimensions;
        max_width = max_width.max(text_dimensions.width);
    }
    let text_height = text_dimensions_arr[0].height;
    let start_y = (screen_height - text_height * messages.len() as f32) / 2.0;
    let start_x = (screen_width - max_width) / 2.0;
    let margin = text_height * 0.2;
    let background_width = max_width + margin * 2.0;
    let background_height = text_height * messages.len() as f32 + margin * 2.0;
    draw_rectangle(
        start_x - margin,
        start_y - margin - text_dimensions_arr[0].offset_y,
        background_width,
        background_height,
        BACKGROUND_COLOR,
    );

    for (line, message) in messages.iter().enumerate() {
        let text_dimensions = text_dimensions_arr[line];

        let x = (screen_width - text_dimensions.width) / 2.0;
        let y = start_y + line as f32 * text_height;

        draw_text_with_shadow(message, x, y, text_size, resource_manager);
    }
}

pub fn draw_centered_text(text: &str, y_coef: f32, size: f32, resource_manager: &ResourceManager) {
    let (width, height) = screen_size();
    let text_size = (size * width) as u16;

    let text_dimensions = measure_text(text, Some(&resource_manager.font), text_size, 1.0);

    let x = (width - text_dimensions.width) / 2.0;
    let y = y_coef * height + text_dimensions.height;

    draw_text_with_shadow(text, x, y, text_size, resource_manager);
}

pub fn draw_loading_screen(font: &Font) {
    let text = "Loading...";
    clear_background(BLACK);
    let (width, height) = screen_size();
    let font_size = (width.max(height) * 0.1) as u16;
    let text_dimenstion = measure_text(text, Some(font), font_size, 1.0);
    let x = (width - text_dimenstion.width) / 2.0;
    let y = height * 0.2;

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size,
            ..Default::default()
        },
    );
}

fn draw_text_with_shadow(
    text: &str,
    x: f32,
    y: f32,
    font_size: u16,
    resource_manager: &ResourceManager,
) {
    draw_text_ex(
        text,
        x + 3.0,
        y + 3.0,
        TextParams {
            font: Some(&resource_manager.font),
            font_size,
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
            font_size,
            font_scale: 1.0,
            color: TEXT_COLOR,
            ..Default::default()
        },
    );
}
