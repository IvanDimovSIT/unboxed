use macroquad::{prelude::info, time::get_frame_time, window::next_frame};

use crate::{
    game_context::GameContext,
    resource_manager::ResourceManager,
    service::{browser, level_loader::load_levels, persistence::load_completed_levels},
    shared_state::SharedState,
};

mod game_context;
mod graphics;
mod input;
mod ivec2;
mod level;
mod level_context;
mod resource_manager;
mod service;
mod shared_state;
#[cfg(test)]
mod tests;
mod ui;

#[macroquad::main("Unboxed")]
async fn main() {
    let resource_manager = ResourceManager::new().await;
    let level_templates = load_levels();
    let completed_levels = load_completed_levels(level_templates.len());
    let is_mobile = browser::is_mobile();
    info!("Is mobile device: {}", is_mobile);

    let mut game_context = GameContext::new(SharedState {
        resource_manager,
        level_templates,
        completed_levels,
        is_mobile,
    });

    loop {
        let delta = get_frame_time();
        game_context.process_frame(delta);
        next_frame().await;
    }
}
