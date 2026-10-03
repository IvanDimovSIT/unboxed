use macroquad::{
    miniquad::window::screen_size,
    text::{TextParams, draw_multiline_text_ex},
};

use crate::{
    game_context::Event,
    graphics::background::draw_background,
    input::{self, get_mouse_vec},
    resource_manager::ResourceManager,
    ui::{buttons::create_back_button, message::draw_centered_text},
};

pub fn draw_help(resource_manager: &ResourceManager) -> Event {
    let (width, height) = screen_size();
    let mut back_button = create_back_button(width, height);
    resource_manager.shader.use_shader(width, height, || {
        draw_background(resource_manager);
        back_button.draw(get_mouse_vec(), resource_manager);
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
        vec![]
    });

    let is_go_to_level_select = back_button.is_clicked || input::exit();

    if is_go_to_level_select {
        Event::ToLevelSelect
    } else {
        Event::None
    }
}
