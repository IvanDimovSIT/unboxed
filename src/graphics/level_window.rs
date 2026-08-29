use macroquad::miniquad::window::screen_size;

use crate::level::Level;

#[derive(Debug)]
pub struct LevelWindowParams {
    pub start_x: f32,
    pub start_y: f32,
    pub width: f32,
    pub _height: f32,
}

pub fn find_level_window_position() -> LevelWindowParams {
    const ASPECT_RATIO: f32 = Level::LEVEL_WIDTH as f32 / Level::LEVEL_HEIGHT as f32;
    const FILL_RATIO: f32 = 0.92;

    let (screen_width, screen_height) = screen_size();

    let available_width = screen_width * FILL_RATIO;
    let available_height = screen_height * FILL_RATIO;

    let (width, height) = if available_width / available_height > ASPECT_RATIO {
        (available_height * ASPECT_RATIO, available_height)
    } else {
        (available_width, available_width / ASPECT_RATIO)
    };

    let start_x = (screen_width - width) / 2.0;
    let start_y = (screen_height - height) / 2.0;

    LevelWindowParams {
        start_x,
        start_y,
        width,
        _height: height,
    }
}
