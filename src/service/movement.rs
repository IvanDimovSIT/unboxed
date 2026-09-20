use std::{collections::HashSet, mem::swap};

use crate::{
    input::MovementInput,
    ivec2::Ivec2,
    level::{AboveTile, Level},
    level_context::LevelContext,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MovementDelta {
    pub tile: AboveTile,
    pub from: Ivec2,
    pub to: Ivec2,
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

    let initial_deltas = if let Some(movement_input) = movement {
        if movement_input == MovementInput::Undo {
            level_context.animation_deltas.clear();
            let mut undo_deltas = level_context.previous_deltas.pop().unwrap_or(vec![]);
            reverse_deltas(&mut undo_deltas);
            apply_deltas(&mut level_context.level, &undo_deltas);
            return ProcessResult::Undo(undo_deltas);
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

    let movement_deltas = remove_dulplicate_deltas(create_movement_deltas(
        &initial_deltas,
        &initial_deltas,
        &level_context.level,
    ));
    if movement_deltas.is_empty() {
        return ProcessResult::None;
    }

    apply_deltas(&mut level_context.level, &movement_deltas);
    ProcessResult::Movement(movement_deltas)
}

fn create_player_deltas(level: &Level, delta: Ivec2) -> Vec<MovementDelta> {
    let mut player_deltas = vec![];
    for y in 0..Level::LEVEL_HEIGHT as i32 {
        for x in 0..Level::LEVEL_WIDTH as i32 {
            let tile = level.get_above(x, y);
            if tile != AboveTile::Player {
                continue;
            }
            let from = Ivec2::new(x, y);
            let to = from + delta;
            player_deltas.push(MovementDelta { tile, from, to });
        }
    }
    let mut deltas = Vec::with_capacity(player_deltas.len() * 2);

    for player_delta in player_deltas {
        deltas.push(player_delta);
        let player_opposite_move_dir = player_delta.from - player_delta.to;
        let mut opposite_tile_pos = player_delta.from + player_opposite_move_dir;
        while !is_position_outside_level(opposite_tile_pos) {
            let opposite_tile = level.get_above(opposite_tile_pos.x, opposite_tile_pos.y);
            if opposite_tile != AboveTile::PullBox {
                break;
            }

            deltas.push(MovementDelta {
                tile: opposite_tile,
                from: opposite_tile_pos,
                to: opposite_tile_pos - player_opposite_move_dir,
            });
            opposite_tile_pos += player_opposite_move_dir;
        }
    }

    deltas
}

fn apply_deltas(level: &mut Level, deltas: &[MovementDelta]) {
    for d in deltas {
        level.set_above(AboveTile::None, d.from.x, d.from.y);
    }
    for d in deltas {
        level.set_above(d.tile, d.to.x, d.to.y);
    }
}

fn create_movement_deltas(
    initial_deltas: &[MovementDelta],
    current_deltas: &[MovementDelta],
    level: &Level,
) -> Vec<MovementDelta> {
    assert!(!current_deltas.is_empty());
    let mut final_deltas = vec![];

    for d in current_deltas {
        if is_position_outside_level(d.to) {
            continue;
        }

        let tile = level.get_above(d.to.x, d.to.y);
        match tile {
            AboveTile::None => {
                final_deltas.push(*d);
            }
            AboveTile::Player | AboveTile::Box => {
                let new_d = MovementDelta {
                    tile,
                    from: d.to,
                    to: calculate_push_delta(d.from, d.to),
                };
                let mut resolved = create_movement_deltas(initial_deltas, &[new_d], level);
                if !resolved.is_empty() {
                    final_deltas.push(*d);
                    final_deltas.append(&mut resolved);
                }
            }
            AboveTile::PullBox => {
                let box_delta_option = initial_deltas.iter().find(|delta| delta.from == d.to);
                if let Some(box_delta) = box_delta_option {
                    let box_dir = box_delta.to - box_delta.from;
                    let movement_dir = d.to - d.from;
                    if box_dir != movement_dir {
                        continue;
                    }
                } else {
                    continue;
                }

                let new_d = MovementDelta {
                    tile,
                    from: d.to,
                    to: calculate_push_delta(d.from, d.to),
                };
                let mut resolved = create_movement_deltas(initial_deltas, &[new_d], level);
                if !resolved.is_empty() {
                    final_deltas.push(*d);
                    final_deltas.append(&mut resolved);
                }
            }
            AboveTile::Wall => {}
        }
    }

    final_deltas
}

fn calculate_push_delta(from: Ivec2, to: Ivec2) -> Ivec2 {
    let d = to - from;
    to + d
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

fn get_direction_for_movement(movement: MovementInput) -> Ivec2 {
    match movement {
        MovementInput::Left => Ivec2::new(-1, 0),
        MovementInput::Right => Ivec2::new(1, 0),
        MovementInput::Down => Ivec2::new(0, 1),
        MovementInput::Up => Ivec2::new(0, -1),
        MovementInput::Undo => panic!("Undo has no direction"),
        MovementInput::Reset => panic!("Reset has no direction"),
    }
}

fn is_position_outside_level(position: Ivec2) -> bool {
    position.x < 0
        || position.x >= Level::LEVEL_WIDTH as i32
        || position.y < 0
        || position.y >= Level::LEVEL_HEIGHT as i32
}
