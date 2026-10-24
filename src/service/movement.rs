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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Action {
    Move(MovementDelta),
    Explosion(Ivec2),
    UndoExplosion(Ivec2),
}

pub fn process(level: &mut Level, movement_input: MovementInput) -> Vec<Action> {
    let direction = get_direction_for_movement(movement_input);
    let initial_movements = create_initial_deltas(level, direction);

    if initial_movements.is_empty() {
        return vec![];
    }

    let result_actions = remove_duplicate_actions(propagate_movements(
        &initial_movements,
        &initial_movements,
        level,
    ));
    if result_actions.is_empty() {
        return vec![];
    }

    apply_actions(level, &result_actions);
    result_actions
}

/// returns the undo actions
pub fn undo_actions(level: &mut Level, mut actions_to_undo: Vec<Action>) -> Vec<Action> {
    if actions_to_undo.is_empty() {
        warn!("Received empty actions to undo");
    }
    reverse_actions(&mut actions_to_undo);
    apply_actions(level, &actions_to_undo);

    actions_to_undo
}

fn create_initial_deltas(level: &Level, delta: Ivec2) -> Vec<MovementDelta> {
    let mut player_deltas = Vec::with_capacity(4);
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
    let mut initial_deltas = Vec::with_capacity(player_deltas.len() * 2);

    for player_delta in player_deltas {
        initial_deltas.push(player_delta);
        let player_opposite_move_dir = player_delta.from - player_delta.to;
        let mut opposite_tile_pos = player_delta.from + player_opposite_move_dir;
        while !is_position_outside_level(opposite_tile_pos) {
            let opposite_tile = level.get_above(opposite_tile_pos.x, opposite_tile_pos.y);
            if opposite_tile != AboveTile::PullBox {
                break;
            }

            initial_deltas.push(MovementDelta {
                tile: opposite_tile,
                from: opposite_tile_pos,
                to: opposite_tile_pos - player_opposite_move_dir,
            });
            opposite_tile_pos += player_opposite_move_dir;
        }
    }

    initial_deltas
}

fn apply_actions(level: &mut Level, actions: &[Action]) {
    for action in actions {
        if let Action::Move(delta) = *action {
            level.set_above(AboveTile::None, delta.from.x, delta.from.y);
        }
    }
    for action in actions {
        if let Action::Move(delta) = *action {
            level.set_above(delta.tile, delta.to.x, delta.to.y);
        }
    }
    for action in actions {
        match action {
            Action::Explosion(ivec2) => level.set_above(AboveTile::None, ivec2.x, ivec2.y),
            Action::UndoExplosion(ivec2) => level.set_above(AboveTile::Bomb, ivec2.x, ivec2.y),
            _ => {}
        }
    }
}

fn propagate_movements(
    initial_movements: &[MovementDelta],
    current_movements: &[MovementDelta],
    level: &Level,
) -> Vec<Action> {
    assert!(!current_movements.is_empty());
    let mut final_actions = Vec::with_capacity(current_movements.len() * 2);

    for current_delta in current_movements {
        if is_position_outside_level(current_delta.to) {
            continue;
        }

        let tile = level.get_above(current_delta.to.x, current_delta.to.y);
        match tile {
            AboveTile::None => {
                final_actions.push(Action::Move(*current_delta));
            }
            AboveTile::Bomb => {
                final_actions.push(Action::Move(*current_delta));
                final_actions.push(Action::Explosion(current_delta.to));
            }
            AboveTile::Player | AboveTile::Box => {
                let new_d = MovementDelta {
                    tile,
                    from: current_delta.to,
                    to: calculate_push_delta(current_delta.from, current_delta.to),
                };
                let mut resolved = propagate_movements(initial_movements, &[new_d], level);
                if !resolved.is_empty() {
                    final_actions.push(Action::Move(*current_delta));
                    final_actions.append(&mut resolved);
                }
            }
            AboveTile::PullBox => {
                push_pull_box(&mut final_actions, *current_delta, initial_movements, level)
            }
            AboveTile::Wall => {}
        }
    }

    final_actions
}

fn push_pull_box(
    final_actions: &mut Vec<Action>,
    push_delta: MovementDelta,
    initial_movements: &[MovementDelta],
    level: &Level,
) {
    let box_delta_option = initial_movements
        .iter()
        .find(|delta| delta.from == push_delta.to);
    if let Some(box_delta) = box_delta_option {
        let box_dir = box_delta.to - box_delta.from;
        let movement_dir = push_delta.to - push_delta.from;
        if box_dir != movement_dir {
            return;
        }
    } else {
        return;
    }

    let new_d = MovementDelta {
        tile: AboveTile::PullBox,
        from: push_delta.to,
        to: calculate_push_delta(push_delta.from, push_delta.to),
    };
    let mut resolved = propagate_movements(initial_movements, &[new_d], level);
    if !resolved.is_empty() {
        final_actions.push(Action::Move(push_delta));
        final_actions.append(&mut resolved);
    }
}

fn calculate_push_delta(from: Ivec2, to: Ivec2) -> Ivec2 {
    let d = to - from;
    to + d
}

fn reverse_actions(actions: &mut [Action]) {
    for action in actions {
        match action {
            Action::Move(movement_delta) => {
                swap(&mut movement_delta.from, &mut movement_delta.to);
            }
            Action::Explosion(ivec2) => *action = Action::UndoExplosion(*ivec2),
            Action::UndoExplosion(ivec2) => {
                panic!("Error, trying to undo an UndoExplosion at {:?}", ivec2)
            }
        }
    }
}

fn remove_duplicate_actions(deltas: Vec<Action>) -> Vec<Action> {
    let set: HashSet<Action> = deltas.into_iter().collect();
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
