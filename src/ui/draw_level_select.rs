use crate::{
    game_context::Event,
    graphics::background::draw_background,
    input::get_mouse_vec,
    resource_manager::ResourceManager,
    ui::{
        buttons::{create_help_button, create_level_button},
        message::draw_centered_text,
    },
};
use macroquad::miniquad::window::screen_size;
use std::collections::HashSet;

const TITLE_Y_COEF: f32 = 0.04;
const TITLE_SIZE_COEF: f32 = 0.08;
const BUTTONS_PER_ROW: usize = 8;
const BUTTONS_SIZE_COEF: f32 = 0.05;
const MARGIN_COEF: f32 = 0.01;

pub fn draw_level_select(
    levels_count: usize,
    completed_levels: &HashSet<usize>,
    resource_manager: &ResourceManager,
) -> Event {
    let (width, height) = screen_size();
    let mut help_btn = create_help_button(width, height);
    let button_size = BUTTONS_SIZE_COEF * width;
    let margin = MARGIN_COEF * width;
    let grid_width = (button_size + margin) * BUTTONS_PER_ROW as f32;
    let start_x = (width - grid_width) / 2.0;
    let start_y = height * 0.23;
    let mut level_buttons = Vec::with_capacity(levels_count);
    for i in 0..levels_count {
        let row = i / BUTTONS_PER_ROW;
        let x = start_x + (i % BUTTONS_PER_ROW) as f32 * (button_size + margin);
        let y = start_y + (button_size + margin) * row as f32;
        let draw_checkmark = completed_levels.contains(&i);
        level_buttons.push(create_level_button(
            i + 1,
            x,
            y,
            button_size,
            draw_checkmark,
        ));
    }

    resource_manager.shader.use_shader(width, height, || {
        let mouse_vec = get_mouse_vec();
        draw_background(resource_manager);
        draw_centered_text("Unboxed", TITLE_Y_COEF, TITLE_SIZE_COEF, resource_manager);
        help_btn.draw(mouse_vec, resource_manager);
        for level_button in &mut level_buttons {
            level_button.draw(mouse_vec, resource_manager);
        }
        vec![]
    });

    let is_go_to_help = help_btn.is_clicked;
    let selected_level = level_buttons
        .into_iter()
        .enumerate()
        .find(|(_level_index, level_btn)| level_btn.is_clicked)
        .map(|(level_index, _level_btn)| level_index);

    if let Some(level) = selected_level {
        Event::ChangeLevel(level)
    } else if is_go_to_help {
        Event::ToHelp
    } else {
        Event::None
    }
}
