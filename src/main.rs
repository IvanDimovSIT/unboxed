use macroquad::{
    color::BLACK,
    time::get_frame_time,
    window::{clear_background, next_frame},
};

use crate::{
    draw::draw_level,
    game_context::GameContext,
    level::{AboveTile, FloorTile, Level, LevelContext},
    service::level_loader::load_levels,
};

mod draw;
mod game_context;
mod input;
mod level;
mod service;

#[macroquad::main("Unboxed")]
async fn main() {
    let levels = load_levels();
    let level_context = LevelContext::new(levels[0].clone());
    let mut game_context = GameContext {
        level_templates: &levels,
        level_context: level_context,
        animation_time_s: 0.0,
        animation_deltas: vec![],
    };

    loop {
        let _delta = get_frame_time();
        clear_background(BLACK);
        let _result = service::movement::process(&mut game_context.level_context);
        draw_level(&game_context.level_context.level, Default::default());
        next_frame().await;
    }
}
