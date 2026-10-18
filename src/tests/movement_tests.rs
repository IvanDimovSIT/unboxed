use super::*;
use std::collections::HashSet;

use crate::{
    input::MovementInput,
    ivec2::Ivec2,
    level::{AboveTile, Level},
    service::movement::{MovementDelta, process, undo_deltas},
};

fn create_delta(tile: AboveTile, from: (i32, i32), to: (i32, i32)) -> MovementDelta {
    MovementDelta {
        tile,
        from: Ivec2::new(from.0, from.1),
        to: Ivec2::new(to.0, to.1),
    }
}

fn assert_deltas(actual: Vec<MovementDelta>, expected: Vec<MovementDelta>) {
    assert_eq!(actual.len(), expected.len());
    let actual: HashSet<MovementDelta> = actual.into_iter().collect();
    let expected: HashSet<MovementDelta> = expected.into_iter().collect();
    assert_eq!(actual, expected);
}

fn assert_blocked(mut level: Level, input: MovementInput) {
    let before = level.clone();
    let deltas = process(&mut level, input);
    assert!(deltas.is_empty());
    assert_eq!(before, level);
}

#[test]
fn test_no_players_means_no_deltas() {
    let mut level = create_level(&[], &[(AboveTile::Box, 5, 5)]);
    let before = level.clone();
    assert!(process(&mut level, MovementInput::Right).is_empty());
    assert_eq!(before, level);
}

#[test]
fn test_player_moves_left() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5)]);
    let deltas = process(&mut level, MovementInput::Left);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (4, 5))],
    );
    assert_eq!(AboveTile::None, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(4, 5));
}

#[test]
fn test_player_moves_right() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5)]);
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (6, 5))],
    );
    assert_eq!(AboveTile::None, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
}

#[test]
fn test_player_moves_up() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5)]);
    let deltas = process(&mut level, MovementInput::Up);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (5, 4))],
    );
    assert_eq!(AboveTile::None, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(5, 4));
}

#[test]
fn test_player_moves_down() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5)]);
    let deltas = process(&mut level, MovementInput::Down);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (5, 6))],
    );
    assert_eq!(AboveTile::None, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(5, 6));
}

#[test]
fn test_player_blocked_by_wall_in_every_direction() {
    let cases = [
        (MovementInput::Left, (4, 5)),
        (MovementInput::Right, (6, 5)),
        (MovementInput::Up, (5, 4)),
        (MovementInput::Down, (5, 6)),
    ];
    for (input, wall) in cases {
        let level = create_level(
            &[],
            &[(AboveTile::Player, 5, 5), (AboveTile::Wall, wall.0, wall.1)],
        );
        assert_blocked(level, input);
    }
}

#[test]
fn test_player_is_not_blocked_by_wall_in_other_directions() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5), (AboveTile::Wall, 6, 5)]);
    let deltas = process(&mut level, MovementInput::Left);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (4, 5))],
    );
}

#[test]
fn test_player_blocked_by_level_edges() {
    let cases = [
        (MovementInput::Left, (0, 5)),
        (MovementInput::Right, (MAX_X, 5)),
        (MovementInput::Up, (5, 0)),
        (MovementInput::Down, (5, MAX_Y)),
    ];
    for (input, pos) in cases {
        let level = create_level(&[], &[(AboveTile::Player, pos.0, pos.1)]);
        assert_blocked(level, input);
    }
}

#[test]
fn test_player_blocked_in_corners() {
    let corner_cases = [
        ((0, 0), MovementInput::Left),
        ((0, 0), MovementInput::Up),
        ((MAX_X, 0), MovementInput::Right),
        ((MAX_X, 0), MovementInput::Up),
        ((0, MAX_Y), MovementInput::Left),
        ((0, MAX_Y), MovementInput::Down),
        ((MAX_X, MAX_Y), MovementInput::Right),
        ((MAX_X, MAX_Y), MovementInput::Down),
    ];
    for (pos, input) in corner_cases {
        assert_blocked(
            create_level(&[], &[(AboveTile::Player, pos.0, pos.1)]),
            input,
        );
    }
}

#[test]
fn test_player_can_move_along_and_into_the_edge() {
    let mut level = create_level(&[], &[(AboveTile::Player, 0, 5)]);
    let deltas = process(&mut level, MovementInput::Up);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (0, 5), (0, 4))],
    );

    let mut level = create_level(&[], &[(AboveTile::Player, MAX_X - 1, 5)]);
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (MAX_X - 1, 5), (MAX_X, 5))],
    );
    assert_eq!(AboveTile::Player, level.get_above(MAX_X, 5));
}

#[test]
fn test_multiple_players_all_move() {
    let mut level = create_level(&[], &[(AboveTile::Player, 2, 2), (AboveTile::Player, 2, 8)]);
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (2, 2), (3, 2)),
            create_delta(AboveTile::Player, (2, 8), (3, 8)),
        ],
    );
    assert_eq!(AboveTile::Player, level.get_above(3, 2));
    assert_eq!(AboveTile::Player, level.get_above(3, 8));
}

#[test]
fn test_blocked_player_does_not_stop_other_players() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 2, 2),
            (AboveTile::Player, 2, 8),
            (AboveTile::Wall, 3, 8),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (2, 2), (3, 2))],
    );
    assert_eq!(AboveTile::Player, level.get_above(3, 2));
    assert_eq!(AboveTile::Player, level.get_above(2, 8));
}

#[test]
fn test_player_pushes_box() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5), (AboveTile::Box, 6, 5)]);
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
            create_delta(AboveTile::Box, (6, 5), (7, 5)),
        ],
    );
    assert_eq!(AboveTile::None, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
    assert_eq!(AboveTile::Box, level.get_above(7, 5));
}

#[test]
fn test_player_pushes_box_in_every_direction() {
    let cases = [
        (MovementInput::Left, (4, 5), (3, 5)),
        (MovementInput::Right, (6, 5), (7, 5)),
        (MovementInput::Up, (5, 4), (5, 3)),
        (MovementInput::Down, (5, 6), (5, 7)),
    ];
    for (input, box_from, box_to) in cases {
        let mut level = create_level(
            &[],
            &[
                (AboveTile::Player, 5, 5),
                (AboveTile::Box, box_from.0, box_from.1),
            ],
        );
        let deltas = process(&mut level, input);
        assert_deltas(
            deltas,
            vec![
                create_delta(AboveTile::Player, (5, 5), box_from),
                create_delta(AboveTile::Box, box_from, box_to),
            ],
        );
        assert_eq!(AboveTile::Player, level.get_above(box_from.0, box_from.1));
        assert_eq!(AboveTile::Box, level.get_above(box_to.0, box_to.1));
        assert_eq!(AboveTile::None, level.get_above(5, 5));
    }
}

#[test]
fn test_player_pushes_chain_of_boxes() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 5, 5),
            (AboveTile::Box, 6, 5),
            (AboveTile::Box, 7, 5),
            (AboveTile::Box, 8, 5),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
            create_delta(AboveTile::Box, (6, 5), (7, 5)),
            create_delta(AboveTile::Box, (7, 5), (8, 5)),
            create_delta(AboveTile::Box, (8, 5), (9, 5)),
        ],
    );
    assert_eq!(AboveTile::None, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
    for x in 7..=9 {
        assert_eq!(AboveTile::Box, level.get_above(x, 5));
    }
}

#[test]
fn test_box_blocked_by_wall() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, 5, 5),
            (AboveTile::Box, 6, 5),
            (AboveTile::Wall, 7, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_chain_of_boxes_blocked_by_wall() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, 5, 5),
            (AboveTile::Box, 6, 5),
            (AboveTile::Box, 7, 5),
            (AboveTile::Wall, 8, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_box_blocked_by_level_edge() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, MAX_X - 1, 5),
            (AboveTile::Box, MAX_X, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);

    let level = create_level(&[], &[(AboveTile::Player, 5, 1), (AboveTile::Box, 5, 0)]);
    assert_blocked(level, MovementInput::Up);
}

#[test]
fn test_chain_of_boxes_blocked_by_level_edge() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, MAX_X - 2, 5),
            (AboveTile::Box, MAX_X - 1, 5),
            (AboveTile::Box, MAX_X, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_box_can_be_pushed_up_to_the_edge() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, MAX_X - 2, 5),
            (AboveTile::Box, MAX_X - 1, 5),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (MAX_X - 2, 5), (MAX_X - 1, 5)),
            create_delta(AboveTile::Box, (MAX_X - 1, 5), (MAX_X, 5)),
        ],
    );
    assert_eq!(AboveTile::Box, level.get_above(MAX_X, 5));
}

#[test]
fn test_box_beside_the_player_is_not_moved() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5), (AboveTile::Box, 5, 4)]);
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (6, 5))],
    );
    assert_eq!(AboveTile::Box, level.get_above(5, 4));
}

#[test]
fn test_box_behind_the_player_is_not_pulled() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5), (AboveTile::Box, 4, 5)]);
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (6, 5))],
    );
    assert_eq!(AboveTile::Box, level.get_above(4, 5));
}

#[test]
fn test_player_pulls_pull_box() {
    let mut level = create_level(
        &[],
        &[(AboveTile::Player, 5, 5), (AboveTile::PullBox, 4, 5)],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
            create_delta(AboveTile::PullBox, (4, 5), (5, 5)),
        ],
    );
    assert_eq!(AboveTile::None, level.get_above(4, 5));
    assert_eq!(AboveTile::PullBox, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
}

#[test]
fn test_player_pulls_pull_box_in_every_direction() {
    let cases = [
        (MovementInput::Left, (6, 5), (4, 5)),
        (MovementInput::Right, (4, 5), (6, 5)),
        (MovementInput::Up, (5, 6), (5, 4)),
        (MovementInput::Down, (5, 4), (5, 6)),
    ];
    for (input, box_from, player_to) in cases {
        let mut level = create_level(
            &[],
            &[
                (AboveTile::Player, 5, 5),
                (AboveTile::PullBox, box_from.0, box_from.1),
            ],
        );
        let deltas = process(&mut level, input);
        assert_deltas(
            deltas,
            vec![
                create_delta(AboveTile::Player, (5, 5), player_to),
                create_delta(AboveTile::PullBox, box_from, (5, 5)),
            ],
        );
        assert_eq!(AboveTile::Player, level.get_above(player_to.0, player_to.1));
        assert_eq!(AboveTile::PullBox, level.get_above(5, 5));
        assert_eq!(AboveTile::None, level.get_above(box_from.0, box_from.1));
    }
}

#[test]
fn test_player_pulls_chain_of_pull_boxes() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 5, 5),
            (AboveTile::PullBox, 4, 5),
            (AboveTile::PullBox, 3, 5),
            (AboveTile::PullBox, 2, 5),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
            create_delta(AboveTile::PullBox, (4, 5), (5, 5)),
            create_delta(AboveTile::PullBox, (3, 5), (4, 5)),
            create_delta(AboveTile::PullBox, (2, 5), (3, 5)),
        ],
    );
    assert_eq!(AboveTile::None, level.get_above(2, 5));
    for x in 3..=5 {
        assert_eq!(AboveTile::PullBox, level.get_above(x, 5));
    }
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
}

#[test]
fn test_pull_box_with_a_gap_behind_the_player_is_not_pulled() {
    let mut level = create_level(
        &[],
        &[(AboveTile::Player, 5, 5), (AboveTile::PullBox, 3, 5)],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (6, 5))],
    );
    assert_eq!(AboveTile::PullBox, level.get_above(3, 5));
}

#[test]
fn test_pull_box_chain_stops_at_a_non_pull_box() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 5, 5),
            (AboveTile::PullBox, 4, 5),
            (AboveTile::Box, 3, 5),
            (AboveTile::PullBox, 2, 5),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
            create_delta(AboveTile::PullBox, (4, 5), (5, 5)),
        ],
    );
    assert_eq!(AboveTile::Box, level.get_above(3, 5));
    assert_eq!(AboveTile::PullBox, level.get_above(2, 5));
}

#[test]
fn test_pull_box_beside_the_player_is_not_moved() {
    let mut level = create_level(
        &[],
        &[(AboveTile::Player, 5, 5), (AboveTile::PullBox, 5, 4)],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 5), (6, 5))],
    );
    assert_eq!(AboveTile::PullBox, level.get_above(5, 4));
}

#[test]
fn test_pull_box_is_not_pulled_when_player_is_blocked_by_wall() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, 5, 5),
            (AboveTile::PullBox, 4, 5),
            (AboveTile::Wall, 6, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_pull_box_is_not_pulled_when_player_is_blocked_by_level_edge() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, MAX_X, 5),
            (AboveTile::PullBox, MAX_X - 1, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_pull_box_cannot_be_pushed() {
    let level = create_level(
        &[],
        &[(AboveTile::Player, 5, 5), (AboveTile::PullBox, 6, 5)],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_pull_box_that_blocks_in_one_direction_is_pulled_in_the_opposite_one() {
    let mut level = create_level(
        &[],
        &[(AboveTile::Player, 5, 5), (AboveTile::PullBox, 6, 5)],
    );
    let deltas = process(&mut level, MovementInput::Left);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 5), (4, 5)),
            create_delta(AboveTile::PullBox, (6, 5), (5, 5)),
        ],
    );
}

#[test]
fn test_player_pushes_box_and_pulls_pull_box_at_the_same_time() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::PullBox, 4, 5),
            (AboveTile::Player, 5, 5),
            (AboveTile::Box, 6, 5),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::PullBox, (4, 5), (5, 5)),
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
            create_delta(AboveTile::Box, (6, 5), (7, 5)),
        ],
    );
    assert_eq!(AboveTile::None, level.get_above(4, 5));
    assert_eq!(AboveTile::PullBox, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
    assert_eq!(AboveTile::Box, level.get_above(7, 5));
}

#[test]
fn test_pushed_box_blocked_also_blocks_the_pull_box() {
    let level = create_level(
        &[],
        &[
            (AboveTile::PullBox, 4, 5),
            (AboveTile::Player, 5, 5),
            (AboveTile::Box, 6, 5),
            (AboveTile::Wall, 7, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_three_players_all_move_in_every_direction() {
    let cases = [
        (MovementInput::Left, (-1, 0)),
        (MovementInput::Right, (1, 0)),
        (MovementInput::Up, (0, -1)),
        (MovementInput::Down, (0, 1)),
    ];
    let positions = [(5, 3), (9, 7), (12, 10)];
    for (input, (dx, dy)) in cases {
        let tiles: Vec<_> = positions
            .iter()
            .map(|&(x, y)| (AboveTile::Player, x, y))
            .collect();
        let mut level = create_level(&[], &tiles);

        let deltas = process(&mut level, input);

        let expected = positions
            .iter()
            .map(|&(x, y)| create_delta(AboveTile::Player, (x, y), (x + dx, y + dy)))
            .collect();
        assert_deltas(deltas, expected);
        for &(x, y) in &positions {
            assert_eq!(AboveTile::None, level.get_above(x, y));
            assert_eq!(AboveTile::Player, level.get_above(x + dx, y + dy));
        }
    }
}

#[test]
fn test_only_the_player_in_front_of_a_wall_is_blocked() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 2, 2),
            (AboveTile::Player, 2, 6),
            (AboveTile::Player, 2, 10),
            (AboveTile::Wall, 3, 6),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (2, 2), (3, 2)),
            create_delta(AboveTile::Player, (2, 10), (3, 10)),
        ],
    );
    assert_eq!(AboveTile::Player, level.get_above(2, 6));
    assert_eq!(AboveTile::Player, level.get_above(3, 2));
    assert_eq!(AboveTile::Player, level.get_above(3, 10));
}

#[test]
fn test_only_the_player_at_the_edge_is_blocked() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, MAX_X, 2),
            (AboveTile::Player, 5, 6),
            (AboveTile::Player, MAX_X, 10),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![create_delta(AboveTile::Player, (5, 6), (6, 6))],
    );
    assert_eq!(AboveTile::Player, level.get_above(MAX_X, 2));
    assert_eq!(AboveTile::Player, level.get_above(MAX_X, 10));
    assert_eq!(AboveTile::Player, level.get_above(6, 6));
}

#[test]
fn test_all_players_blocked_by_walls_and_edges() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, 2, 2),
            (AboveTile::Wall, 3, 2),
            (AboveTile::Player, MAX_X, 6),
            (AboveTile::Player, 8, 10),
            (AboveTile::Wall, 9, 10),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_adjacent_players_in_a_row_move_together() {
    let mut level = create_level(&[], &[(AboveTile::Player, 5, 5), (AboveTile::Player, 6, 5)]);
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
            create_delta(AboveTile::Player, (6, 5), (7, 5)),
        ],
    );
    assert_eq!(AboveTile::None, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
    assert_eq!(AboveTile::Player, level.get_above(7, 5));
}

#[test]
fn test_adjacent_players_blocked_by_wall_in_front_of_the_leader() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, 5, 5),
            (AboveTile::Player, 6, 5),
            (AboveTile::Wall, 7, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_adjacent_players_blocked_by_edge_in_front_of_the_leader() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, MAX_X - 1, 5),
            (AboveTile::Player, MAX_X, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_player_pushes_box_into_another_player() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 3, 5),
            (AboveTile::Box, 4, 5),
            (AboveTile::Player, 5, 5),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (3, 5), (4, 5)),
            create_delta(AboveTile::Box, (4, 5), (5, 5)),
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
        ],
    );
    assert_eq!(AboveTile::None, level.get_above(3, 5));
    assert_eq!(AboveTile::Player, level.get_above(4, 5));
    assert_eq!(AboveTile::Box, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
}

#[test]
fn test_player_pushes_box_into_another_player_who_is_blocked() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, 3, 5),
            (AboveTile::Box, 4, 5),
            (AboveTile::Player, 5, 5),
            (AboveTile::Wall, 6, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_each_player_pushes_own_box_and_only_the_blocked_one_stays() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 2, 2),
            (AboveTile::Box, 3, 2),
            (AboveTile::Player, 2, 8),
            (AboveTile::Box, 3, 8),
            (AboveTile::Wall, 4, 8),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (2, 2), (3, 2)),
            create_delta(AboveTile::Box, (3, 2), (4, 2)),
        ],
    );
    assert_eq!(AboveTile::Player, level.get_above(3, 2));
    assert_eq!(AboveTile::Box, level.get_above(4, 2));
    assert_eq!(AboveTile::Player, level.get_above(2, 8));
    assert_eq!(AboveTile::Box, level.get_above(3, 8));
    assert_eq!(AboveTile::Wall, level.get_above(4, 8));
}

#[test]
fn test_each_player_pulls_own_pull_box() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::PullBox, 4, 2),
            (AboveTile::Player, 5, 2),
            (AboveTile::PullBox, 4, 8),
            (AboveTile::Player, 5, 8),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 2), (6, 2)),
            create_delta(AboveTile::PullBox, (4, 2), (5, 2)),
            create_delta(AboveTile::Player, (5, 8), (6, 8)),
            create_delta(AboveTile::PullBox, (4, 8), (5, 8)),
        ],
    );
    for y in [2, 8] {
        assert_eq!(AboveTile::None, level.get_above(4, y));
        assert_eq!(AboveTile::PullBox, level.get_above(5, y));
        assert_eq!(AboveTile::Player, level.get_above(6, y));
    }
}

#[test]
fn test_blocked_player_does_not_pull_its_pull_box_but_the_other_player_does() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::PullBox, 4, 2),
            (AboveTile::Player, 5, 2),
            (AboveTile::PullBox, 4, 8),
            (AboveTile::Player, 5, 8),
            (AboveTile::Wall, 6, 8),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (5, 2), (6, 2)),
            create_delta(AboveTile::PullBox, (4, 2), (5, 2)),
        ],
    );
    assert_eq!(AboveTile::PullBox, level.get_above(5, 2));
    assert_eq!(AboveTile::Player, level.get_above(6, 2));
    assert_eq!(AboveTile::PullBox, level.get_above(4, 8));
    assert_eq!(AboveTile::Player, level.get_above(5, 8));
}

#[test]
fn test_pull_box_between_two_players_moves_with_both() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 3, 5),
            (AboveTile::PullBox, 4, 5),
            (AboveTile::Player, 5, 5),
        ],
    );
    let deltas = process(&mut level, MovementInput::Right);
    assert_deltas(
        deltas,
        vec![
            create_delta(AboveTile::Player, (3, 5), (4, 5)),
            create_delta(AboveTile::PullBox, (4, 5), (5, 5)),
            create_delta(AboveTile::Player, (5, 5), (6, 5)),
        ],
    );
    assert_eq!(AboveTile::None, level.get_above(3, 5));
    assert_eq!(AboveTile::Player, level.get_above(4, 5));
    assert_eq!(AboveTile::PullBox, level.get_above(5, 5));
    assert_eq!(AboveTile::Player, level.get_above(6, 5));
}

#[test]
fn test_pull_box_between_two_players_blocked_when_front_player_is_blocked() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, 3, 5),
            (AboveTile::PullBox, 4, 5),
            (AboveTile::Player, 5, 5),
            (AboveTile::Wall, 6, 5),
        ],
    );
    assert_blocked(level, MovementInput::Right);
}

#[test]
fn test_undoing_deltas_of_multiple_players_restores_the_level() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::Player, 2, 2),
            (AboveTile::Box, 3, 2),
            (AboveTile::PullBox, 1, 8),
            (AboveTile::Player, 2, 8),
            (AboveTile::Player, 2, 12),
            (AboveTile::Wall, 3, 12),
        ],
    );
    let before = level.clone();

    let deltas = process(&mut level, MovementInput::Right);
    assert_eq!(4, deltas.len());
    assert_ne!(before, level);

    undo_deltas(&mut level, deltas);
    assert_eq!(before, level);
}

#[test]
fn test_undoing_deltas_restores_the_level_and_returns_reversed_deltas() {
    let mut level = create_level(
        &[],
        &[
            (AboveTile::PullBox, 4, 5),
            (AboveTile::Player, 5, 5),
            (AboveTile::Box, 6, 5),
            (AboveTile::Wall, 10, 10),
        ],
    );
    let before = level.clone();

    let deltas = process(&mut level, MovementInput::Right);
    assert_eq!(3, deltas.len());
    assert_ne!(before, level);

    let undo = undo_deltas(&mut level, deltas.clone());
    assert_eq!(before, level);

    let expected_undo: Vec<MovementDelta> = deltas
        .iter()
        .map(|delta| MovementDelta {
            tile: delta.tile,
            from: delta.to,
            to: delta.from,
        })
        .collect();
    assert_deltas(undo, expected_undo);
}
