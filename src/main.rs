use macroquad::{
    color::BLACK,
    time::get_frame_time,
    window::{clear_background, next_frame},
};

use crate::{
    draw::{LevelDrawContext, draw_level},
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
    let animation_time = 0.13;

    loop {
        let delta = get_frame_time();

        clear_background(BLACK);
        let result = service::movement::process(&mut game_context.level_context);
        match result {
            service::movement::ProcessResult::None => {}
            service::movement::ProcessResult::Movement(movement_deltas) => {
                game_context.animation_deltas = movement_deltas.clone();
                game_context.animation_time_s = 0.0;
                game_context
                    .level_context
                    .previous_deltas
                    .push(movement_deltas);
            }
            service::movement::ProcessResult::Undo(undo_movement_deltas) => {
                game_context.animation_deltas = undo_movement_deltas;
                game_context.animation_time_s = 0.0;
            }
            service::movement::ProcessResult::Reset => {
                game_context.animation_deltas = vec![];
                game_context.animation_time_s = 0.0;
            }
        }

        game_context.animation_time_s += delta;
        if game_context.animation_time_s >= animation_time {
            game_context.animation_deltas = vec![];
            game_context.animation_time_s = 0.0;
        }

        let animation_progress = game_context.animation_time_s / animation_time;
        draw_level(
            &game_context.level_context.level,
            &game_context.animation_deltas,
            LevelDrawContext {
                animation_progress,
                ..Default::default()
            },
        );
        next_frame().await;
    }
}
