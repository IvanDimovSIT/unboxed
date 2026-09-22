use std::{collections::HashSet, mem::swap};

use macroquad::prelude::warn;

use crate::{
    input::MovementInput,
    ivec2::Ivec2,
    level::{AboveTile, Level},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MovementDelta {
    pub tile: AboveTile,
    pub from: Ivec2,
    pub to: Ivec2,
}

pub fn process(level: &mut Level, movement_input: MovementInput) -> Vec<MovementDelta> {
    let direction = get_direction_for_movement(movement_input);
    let initial_deltas = create_initial_deltas(&level, direction);

    if initial_deltas.is_empty() {
        return vec![];
    }

    let movement_deltas = remove_dulplicate_deltas(create_movement_deltas(
        &initial_deltas,
        &initial_deltas,
        level,
    ));
    if movement_deltas.is_empty() {
        return vec![];
    }

    apply_deltas(level, &movement_deltas);
    movement_deltas
}

/// returns the undo deltas
pub fn undo_deltas(
    level: &mut Level,
    mut deltas_to_undo: Vec<MovementDelta>,
) -> Vec<MovementDelta> {
    if deltas_to_undo.is_empty() {
        warn!("Received empty deltas to undo");
    }
    reverse_deltas(&mut deltas_to_undo);
    apply_deltas(level, &mut deltas_to_undo);

    deltas_to_undo
}

fn create_initial_deltas(level: &Level, delta: Ivec2) -> Vec<MovementDelta> {
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
