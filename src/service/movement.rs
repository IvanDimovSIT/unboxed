use std::collections::HashSet;

use crate::{
    input,
    level::{AboveTile, Level, LevelContext},
};

#[derive(Debug, Clone, Copy)]
pub struct MovementDelta {
    pub tile: AboveTile,
    pub from: (i32, i32),
    pub to: (i32, i32),
}

#[derive(Debug, Clone)]
pub enum ProcessResult {
    None,
    Movement(Vec<MovementDelta>),
    Reset,
}

pub fn process(level_context: &mut LevelContext) -> ProcessResult {
    let initial_deltas = if input::left() {
        create_player_deltas(&level_context.level, (-1, 0))
    } else if input::right() {
        create_player_deltas(&level_context.level, (1, 0))
    } else if input::up() {
        create_player_deltas(&level_context.level, (0, -1))
    } else if input::down() {
        create_player_deltas(&level_context.level, (0, 1))
    } else {
        vec![]
    };

    if input::reset() {
        level_context.reset();
        return ProcessResult::Reset;
    }

    if !initial_deltas.is_empty() {
        let movement_deltas = create_movement_deltas(initial_deltas, &level_context.level);
        // TODO: filter duplicates?
        if !movement_deltas.is_empty() {
            apply_deltas(&mut level_context.level, &movement_deltas);
            ProcessResult::Movement(movement_deltas)
        } else {
            ProcessResult::None
        }
    } else {
        ProcessResult::None
    }
}

fn create_player_deltas(level: &Level, delta: (i32, i32)) -> Vec<MovementDelta> {
    let mut deltas = vec![];
    for y in 0..Level::LEVEL_SIZE as i32 {
        for x in 0..Level::LEVEL_SIZE as i32 {
            let tile = level.get_above(x, y);
            if tile != AboveTile::Player {
                continue;
            }
            deltas.push(MovementDelta {
                tile,
                from: (x, y),
                to: (x + delta.0, y + delta.1),
            });
        }
    }

    deltas
}

fn apply_deltas(level: &mut Level, deltas: &[MovementDelta]) {
    for d in deltas {
        let (x, y) = d.from;
        level.set_above(AboveTile::None, x, y);
    }
    for d in deltas {
        let (x, y) = d.to;
        level.set_above(d.tile, x, y);
    }
}

fn create_movement_deltas(initial: Vec<MovementDelta>, level: &Level) -> Vec<MovementDelta> {
    assert!(!initial.is_empty());
    let mut final_delta = vec![];

    for d in &initial {
        let (x_to, y_to) = d.to;
        if x_to < 0 || x_to >= Level::LEVEL_SIZE as i32 {
            continue;
        }
        if y_to < 0 || y_to >= Level::LEVEL_SIZE as i32 {
            continue;
        }
        // TODO: check for duplicates or conflicts??

        let tile = level.get_above(x_to, y_to);
        match tile {
            AboveTile::None => {
                final_delta.push(*d);
            }
            AboveTile::Player | AboveTile::Box => {
                let new_d = MovementDelta {
                    tile,
                    from: d.to,
                    to: calculate_push_delta(d.from, d.to),
                };
                let mut resolved = create_movement_deltas(vec![new_d], level);
                if !resolved.is_empty() {
                    final_delta.push(*d);
                    final_delta.append(&mut resolved);
                }
            }
            AboveTile::Wall => {}
        }
    }

    final_delta
}

fn calculate_push_delta(from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
    let d = (to.0 - from.0, to.1 - from.1);
    (to.0 + d.0, to.1 + d.1)
}
