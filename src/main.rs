use macroquad::{time::get_frame_time, window::next_frame};

use crate::{
    game_context::GameContext, resource_manager::ResourceManager,
    service::level_loader::load_levels,
};

mod game_context;
mod graphics;
mod input;
mod level;
mod resource_manager;
mod service;
mod ui;

#[macroquad::main("Unboxed")]
async fn main() {
    let resource_manager = ResourceManager::new();
    let levels = load_levels();
    let mut game_context = GameContext::new(&resource_manager, &levels);

    loop {
        let delta = get_frame_time();
        game_context.process_frame(delta);
        next_frame().await;
    }
}
