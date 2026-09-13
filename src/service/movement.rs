use std::{collections::HashSet, mem::swap};

use crate::{
    input::MovementInput,
    level::{AboveTile, Level},
    level_context::LevelContext,
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

pub fn process(level_context: &mut LevelContext, movement: Option<MovementInput>) -> ProcessResult {
    if level_context.is_win {
        return ProcessResult::None;
    }

    let mut is_undo = false;
    let initial_deltas = if let Some(movement_input) = movement {
        if movement_input == MovementInput::Undo {
            level_context.animation_deltas.clear();
            let mut undo_deltas = level_context.previous_deltas.pop().unwrap_or(vec![]);
            reverse_deltas(&mut undo_deltas);
            is_undo = true;
            undo_deltas
        } else if movement_input == MovementInput::Reset {
            level_context.reset();
            return ProcessResult::Reset;
        } else {
            let direction = get_direction_for_movement(movement_input);
            create_player_deltas(&level_context.level, direction)
        }
    } else {
        vec![]
    };

    if initial_deltas.is_empty() {
        return ProcessResult::None;
    }

    let movement_deltas =
        remove_dulplicate_deltas(create_movement_deltas(initial_deltas, &level_context.level));
    if movement_deltas.is_empty() {
        return ProcessResult::None;
    }

    apply_deltas(&mut level_context.level, &movement_deltas);
    if is_undo {
        ProcessResult::Undo(movement_deltas)
    } else {
        ProcessResult::Movement(movement_deltas)
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

fn get_direction_for_movement(movement: MovementInput) -> (i32, i32) {
    match movement {
        MovementInput::Left => (-1, 0),
        MovementInput::Right => (1, 0),
        MovementInput::Down => (0, 1),
        MovementInput::Up => (0, -1),
        MovementInput::Undo => panic!("Undo has no direction"),
        MovementInput::Reset => panic!("Reset has no direction"),
    }
}
