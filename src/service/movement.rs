use std::{collections::HashSet, mem::swap};

use crate::{
    input,
    level::{AboveTile, Level, LevelContext},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    Undo(Vec<MovementDelta>),
}

pub fn process(level_context: &mut LevelContext) -> ProcessResult {
    if level_context.is_win || level_context.animation_deltas.len() > 0 {
        return ProcessResult::None;
    }

    let mut is_undo = false;
    let initial_deltas = if input::left() {
        create_player_deltas(&level_context.level, (-1, 0))
    } else if input::right() {
        create_player_deltas(&level_context.level, (1, 0))
    } else if input::up() {
        create_player_deltas(&level_context.level, (0, -1))
    } else if input::down() {
        create_player_deltas(&level_context.level, (0, 1))
    } else if input::undo() {
        let mut undo_deltas = level_context.previous_deltas.pop().unwrap_or(vec![]);
        reverse_deltas(&mut undo_deltas);
        is_undo = true;
        undo_deltas
    } else {
        vec![]
    };

    if input::reset() {
        level_context.reset();
        return ProcessResult::Reset;
    }

    if !initial_deltas.is_empty() {
        let movement_deltas =
            remove_dulplicate_deltas(create_movement_deltas(initial_deltas, &level_context.level));
        if !movement_deltas.is_empty() {
            apply_deltas(&mut level_context.level, &movement_deltas);
            if is_undo {
                ProcessResult::Undo(movement_deltas)
            } else {
                ProcessResult::Movement(movement_deltas)
            }
        } else {
            ProcessResult::None
        }
    } else {
        ProcessResult::None
    }
}

fn create_player_deltas(level: &Level, delta: (i32, i32)) -> Vec<MovementDelta> {
    let mut deltas = vec![];
    for y in 0..Level::LEVEL_HEIGHT as i32 {
        for x in 0..Level::LEVEL_WIDTH as i32 {
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
        if x_to < 0 || x_to >= Level::LEVEL_WIDTH as i32 {
            continue;
        }
        if y_to < 0 || y_to >= Level::LEVEL_WIDTH as i32 {
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

fn reverse_deltas(deltas: &mut [MovementDelta]) {
    for d in deltas {
        swap(&mut d.from, &mut d.to);
    }
}

fn remove_dulplicate_deltas(deltas: Vec<MovementDelta>) -> Vec<MovementDelta> {
    let set: HashSet<MovementDelta> = deltas.into_iter().collect();
    set.into_iter().collect()
}
