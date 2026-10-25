use macroquad::miniquad::window::screen_size;

use crate::{level::Level, resource_manager::ResourceManager};

#[derive(Debug)]
pub struct InnerWindowParams {
    pub start_x: f32,
    pub start_y: f32,
    pub width: f32,
    pub height: f32,
}

pub fn find_level_window_position() -> InnerWindowParams {
    const ASPECT_RATIO: f32 = Level::LEVEL_WIDTH as f32 / Level::LEVEL_HEIGHT as f32;
    const FILL_RATIO: f32 = 0.92;

    find_inner_window_position(ASPECT_RATIO, FILL_RATIO)
}

pub fn find_rotate_screen_prompt_position(resource_manager: &ResourceManager) -> InnerWindowParams {
    const FILL_RATIO: f32 = 0.8;

    let rotate_device_prompt_img = &resource_manager.rotate_device_prompt;
    let aspect_ratio = rotate_device_prompt_img.width() / rotate_device_prompt_img.height();

    find_inner_window_position(aspect_ratio, FILL_RATIO)
}

fn find_inner_window_position(aspect_ratio: f32, fill_ratio: f32) -> InnerWindowParams {
    assert!(aspect_ratio > 0.0);
    let (screen_width, screen_height) = screen_size();

    let available_width = screen_width * fill_ratio;
    let available_height = screen_height * fill_ratio;

    let (width, height) = if available_width / available_height > aspect_ratio {
        (available_height * aspect_ratio, available_height)
    } else {
        (available_width, available_width / aspect_ratio)
    };

    let start_x = (screen_width - width) / 2.0;
    let start_y = (screen_height - height) / 2.0;

    InnerWindowParams {
        start_x,
        start_y,
        width,
        height,
    }
}
