use macroquad::{
    miniquad::window::screen_size,
    text::{TextParams, draw_multiline_text_ex},
};

use crate::{
    game_context::Event,
    graphics::background::draw_background,
    input,
    resource_manager::ResourceManager,
    ui::{buttons::draw_back_button, message::draw_centered_text},
};

pub fn draw_help(resource_manager: &ResourceManager) -> Event {
    let (width, height) = screen_size();
    draw_background(resource_manager);

    let is_go_to_level_select = draw_back_button(resource_manager) || input::exit();

    draw_centered_text("Controls", 0.04, 0.08, resource_manager);

    let x = width * 0.1;
    let y = height * 0.3;
    let font_size = (height * 0.05).round() as u16;
    draw_multiline_text_ex(
        "W/S/A/D - Move\nZ - Undo\nR - Reset\nESC - Back",
        x,
        y,
        None,
        TextParams {
            font: Some(&resource_manager.font),
            font_size,
            ..Default::default()
        },
    );

    if is_go_to_level_select {
        Event::ToLevelSelect
    } else {
        Event::None
    }
}
