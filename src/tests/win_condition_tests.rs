use super::*;
use crate::{
    level::{AboveTile, FloorTile, Level},
    service::win_condition::is_win,
};

#[test]
fn test_level_without_exits_is_a_win() {
    assert!(is_win(&Level::default()));
}

#[test]
fn test_tiles_above_without_exits_do_not_matter() {
    let level = create_level(
        &[],
        &[
            (AboveTile::Player, 1, 1),
            (AboveTile::Box, 2, 2),
            (AboveTile::PullBox, 3, 3),
            (AboveTile::Wall, 4, 4),
        ],
    );
    assert!(is_win(&level));
}

#[test]
fn test_player_on_player_exit_is_a_win() {
    let level = create_level(
        &[(FloorTile::PlayerExit, 5, 5)],
        &[(AboveTile::Player, 5, 5)],
    );
    assert!(is_win(&level));
}

#[test]
fn test_empty_player_exit_is_not_a_win() {
    let level = create_level(&[(FloorTile::PlayerExit, 5, 5)], &[]);
    assert!(!is_win(&level));
}

#[test]
fn test_player_next_to_player_exit_is_not_a_win() {
    let level = create_level(
        &[(FloorTile::PlayerExit, 5, 5)],
        &[(AboveTile::Player, 4, 5)],
    );
    assert!(!is_win(&level));
}

#[test]
fn test_box_on_player_exit_is_not_a_win() {
    for tile in [AboveTile::Box, AboveTile::PullBox, AboveTile::Wall] {
        let level = create_level(&[(FloorTile::PlayerExit, 5, 5)], &[(tile, 5, 5)]);
        assert!(!is_win(&level));
    }
}

#[test]
fn test_box_on_box_exit_is_a_win() {
    let level = create_level(&[(FloorTile::BoxExit, 5, 5)], &[(AboveTile::Box, 5, 5)]);
    assert!(is_win(&level));
}

#[test]
fn test_pull_box_on_box_exit_is_a_win() {
    let level = create_level(&[(FloorTile::BoxExit, 5, 5)], &[(AboveTile::PullBox, 5, 5)]);
    assert!(is_win(&level));
}

#[test]
fn test_empty_box_exit_is_not_a_win() {
    let level = create_level(&[(FloorTile::BoxExit, 5, 5)], &[]);
    assert!(!is_win(&level));
}

#[test]
fn test_player_or_wall_on_box_exit_is_not_a_win() {
    for tile in [AboveTile::Player, AboveTile::Wall] {
        let level = create_level(&[(FloorTile::BoxExit, 5, 5)], &[(tile, 5, 5)]);
        assert!(!is_win(&level));
    }
}

#[test]
fn test_box_next_to_box_exit_is_not_a_win() {
    let level = create_level(&[(FloorTile::BoxExit, 5, 5)], &[(AboveTile::Box, 6, 5)]);
    assert!(!is_win(&level));
}

#[test]
fn test_all_exits_satisfied_is_a_win() {
    let level = create_level(
        &[
            (FloorTile::PlayerExit, 2, 2),
            (FloorTile::PlayerExit, 3, 7),
            (FloorTile::BoxExit, 8, 4),
            (FloorTile::BoxExit, 9, 9),
        ],
        &[
            (AboveTile::Player, 2, 2),
            (AboveTile::Player, 3, 7),
            (AboveTile::Box, 8, 4),
            (AboveTile::PullBox, 9, 9),
        ],
    );
    assert!(is_win(&level));
}

#[test]
fn test_one_unsatisfied_box_exit_is_not_a_win() {
    let level = create_level(
        &[
            (FloorTile::PlayerExit, 2, 2),
            (FloorTile::BoxExit, 8, 4),
            (FloorTile::BoxExit, 9, 9),
        ],
        &[(AboveTile::Player, 2, 2), (AboveTile::Box, 8, 4)],
    );
    assert!(!is_win(&level));
}

#[test]
fn test_one_unsatisfied_player_exit_is_not_a_win() {
    let level = create_level(
        &[
            (FloorTile::PlayerExit, 2, 2),
            (FloorTile::PlayerExit, 3, 7),
            (FloorTile::BoxExit, 8, 4),
        ],
        &[(AboveTile::Player, 2, 2), (AboveTile::Box, 8, 4)],
    );
    assert!(!is_win(&level));
}

#[test]
fn test_satisfied_box_exit_does_not_make_up_for_empty_player_exit() {
    let level = create_level(
        &[(FloorTile::PlayerExit, 2, 2), (FloorTile::BoxExit, 8, 4)],
        &[(AboveTile::Box, 8, 4)],
    );
    assert!(!is_win(&level));
}

#[test]
fn test_satisfied_player_exit_does_not_make_up_for_empty_box_exit() {
    let level = create_level(
        &[(FloorTile::PlayerExit, 2, 2), (FloorTile::BoxExit, 8, 4)],
        &[(AboveTile::Player, 2, 2)],
    );
    assert!(!is_win(&level));
}

#[test]
fn test_extra_boxes_and_players_off_exits_do_not_matter() {
    let level = create_level(
        &[(FloorTile::PlayerExit, 2, 2), (FloorTile::BoxExit, 8, 4)],
        &[
            (AboveTile::Player, 2, 2),
            (AboveTile::Box, 8, 4),
            (AboveTile::Player, 10, 10),
            (AboveTile::Box, 11, 11),
            (AboveTile::PullBox, 12, 12),
            (AboveTile::Wall, 13, 13),
        ],
    );
    assert!(is_win(&level));
}

#[test]
fn test_exits_in_every_corner_are_checked() {
    let corners = [(0, 0), (MAX_X, 0), (0, MAX_Y), (MAX_X, MAX_Y)];

    let floors: Vec<_> = corners
        .iter()
        .map(|&(x, y)| (FloorTile::BoxExit, x, y))
        .collect();
    let all_boxes: Vec<_> = corners
        .iter()
        .map(|&(x, y)| (AboveTile::Box, x, y))
        .collect();
    assert!(is_win(&create_level(&floors, &all_boxes)));

    for skipped in 0..corners.len() {
        let partial: Vec<_> = all_boxes
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != skipped)
            .map(|(_, &tile)| tile)
            .collect();
        assert!(!is_win(&create_level(&floors, &partial)));
    }
}

#[test]
fn test_every_tile_as_an_exit_is_a_win_only_when_all_are_filled() {
    let mut floors = vec![];
    let mut aboves = vec![];
    for y in 0..=MAX_Y {
        for x in 0..=MAX_X {
            floors.push((FloorTile::BoxExit, x, y));
            aboves.push((AboveTile::Box, x, y));
        }
    }
    assert!(is_win(&create_level(&floors, &aboves)));

    aboves.pop();
    assert!(!is_win(&create_level(&floors, &aboves)));
}
