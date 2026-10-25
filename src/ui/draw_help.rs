use macroquad::{
    miniquad::window::screen_size,
    text::{TextParams, draw_multiline_text_ex},
};

use crate::{
    game_context::Event,
    graphics::background::draw_background,
    input::{self, get_mouse_vec},
    shared_state::SharedState,
    ui::{buttons::Button, message::draw_centered_text},
};

const FONT_SIZE_COEF: f32 = 0.06;

pub fn draw_help(shared_state: &SharedState) -> Event {
    let (width, height) = screen_size();
    let mut back_button = Button::new_back(width, height, shared_state.is_mobile);
    let resource_manager = &shared_state.resource_manager;
    resource_manager.shader.use_shader(width, height, || {
        draw_background(&shared_state.resource_manager);
        back_button.detect(get_mouse_vec(), shared_state);
        back_button.draw(resource_manager);
        draw_centered_text("Controls", 0.04, 0.08, resource_manager);
        let x = width * 0.1;
        let y = height * 0.3;
        let font_size = (height * FONT_SIZE_COEF).round() as u16;
        let text = if shared_state.is_mobile {
            "Swipe or tap the sides of the screen to move\nUndo with the top right button\nReset the level with the X button"
        } else {
            "W/S/A/D - Move\nZ - Undo\nR - Reset\nESC - Back"
        };

        draw_multiline_text_ex(
            text,
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
