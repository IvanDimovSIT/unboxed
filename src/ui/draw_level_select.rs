use macroquad::{
    color::BLACK, input::mouse_position, math::vec2, miniquad::window::screen_size,
    window::clear_background,
};

use crate::{
    game_context::Event,
    resource_manager::ResourceManager,
    ui::{
        level_button::{DrawLevelButtonContext, draw_level_button},
        message::draw_centered_text,
    },
};

const TITLE_Y_COEF: f32 = 0.04;
const TITLE_SIZE_COEF: f32 = 0.08;
const BUTTONS_PER_ROW: usize = 8;
const BUTTONS_SIZE_COEF: f32 = 0.05;
const MARGIN_COEF: f32 = 0.01;

pub fn draw_level_select(levels_count: usize, resource_manager: &ResourceManager) -> Event {
    clear_background(BLACK);
    let (width, height) = screen_size();
    let button_size = BUTTONS_SIZE_COEF * width;
    let margin = MARGIN_COEF * width;
    draw_centered_text("Unboxed", TITLE_Y_COEF, TITLE_SIZE_COEF, resource_manager);
    let (mouse_x, mouse_y) = mouse_position();
    let grid_width = (button_size + margin) * BUTTONS_PER_ROW as f32;
    let start_x = (width - grid_width) / 2.0;
    let start_y = height * 0.2;

    let button_ctx = DrawLevelButtonContext {
        size: button_size,
        resource_manager,
        mouse_pos: vec2(mouse_x, mouse_y),
    };

    let mut selected_level = None;
    for i in 0..levels_count {
        let row = i / BUTTONS_PER_ROW;
        let x = start_x + (i % BUTTONS_PER_ROW) as f32 * (button_size + margin);
        let y = start_y + (button_size + margin) * row as f32;
        if draw_level_button(i, x, y, &button_ctx) {
            selected_level = Some(i);
        }
    }

    if let Some(level) = selected_level {
        Event::ChangeLevel(level)
    } else {
        Event::None
    }
}
