use std::sync::LazyLock;

use crate::{
    input::MovementInput,
    level::Level,
    service::{
        level_loader,
        movement::{self, Action},
        win_condition::is_win,
    },
};

static LEVELS: LazyLock<Vec<Level>> = LazyLock::new(level_loader::load_levels);

fn test_level_solution(level_num: usize, input_codes: &str) {
    let level_index = level_num - 1;
    let inputs = MovementInput::from_string(input_codes);
    let mut level = LEVELS[level_index].clone();
    let original_level = level.clone();

    let mut actions_vec = Vec::with_capacity(inputs.len());
    for input in inputs {
        assert!(!is_win(&level), "Level has been won earlier than expected");
        let level_copy = level.clone();
        let result_actions = movement::process(&mut level, input);
        validate_undo(level.clone(), &level_copy, result_actions.clone());
        actions_vec.push(result_actions);
    }
    assert!(is_win(&level), "Level has not been won");

    validate_full_undo(level, &original_level, actions_vec);
}

fn validate_undo(
    mut current_level_state: Level,
    previous_level_state: &Level,
    actions_to_undo: Vec<Action>,
) {
    movement::undo_actions(&mut current_level_state, actions_to_undo);
    assert_eq!(*previous_level_state, current_level_state, "Undo failed");
}

fn validate_full_undo(
    mut end_level_state: Level,
    original_level_state: &Level,
    actions_vec: Vec<Vec<Action>>,
) {
    for actions_to_undo in actions_vec.into_iter().rev() {
        movement::undo_actions(&mut end_level_state, actions_to_undo);
    }
    assert_eq!(
        *original_level_state, end_level_state,
        "Undo from end to start failed"
    );
}

#[test]
#[should_panic]
fn test_level1_losing() {
    let inputs = "rrruurdldrrrrdruu";
    test_level_solution(1, inputs);
}

#[test]
fn test_level1() {
    let inputs = "rrrurdldrrrrrdruuu";
    test_level_solution(1, inputs);
}

#[test]
fn test_level2() {
    let inputs = "ddrrrrrrdrruuudddllurdruudlllllllrrlldrrrrrrrdrruuudddllurdruu";
    test_level_solution(2, inputs);
}

#[test]
fn test_level3() {
    let inputs = "rrrrrrrdrdruluurdddllurdruu";
    test_level_solution(3, inputs);
}

#[test]
fn test_level4() {
    let inputs = "llrrrrldluudldluruldlururrddrdllllrurdllluurrrrrr";
    test_level_solution(4, inputs);
}

#[test]
fn test_level5() {
    let inputs = "llullddldrrluurrrdrrdddddllllrdulldluururrrur";
    test_level_solution(5, inputs);
}

#[test]
fn test_level6() {
    let inputs = "ullllrurlurlruurrrulllluurrrurrulllrrurrrddlddddldddrrrlrurrurrddddrrr";
    test_level_solution(6, inputs);
}

#[test]
fn test_level7() {
    let inputs = "rruuurrrrrddddrurllduuurrulllldlllrullllrlldlddddurldrurdurdlldrrrlulururuullururdllllrldddurdrrdrrrrdldruuuurrrrrddddrru";
    test_level_solution(7, inputs);
}

#[test]
fn test_level8() {
    let inputs = "llllllllluuuulurrdddddldddddrurrrrudlllluuuuuuddrruulrulurrddruluurrdlllllrrrddllluluuuuuurrrrddrdlldddrrdrddrrrrurrrrdlllllllluuuuuulluruldlurdrruruldrdddddrrrrrrurullllllrrrddlllluuuuuulluuuurdrrrddrdlldddrrrrrrrruluurrdlldllluuurrrdlllllldrdrdrdlldllllluuuuuddddrdrrrddrdrrrrrrrrrdllllllullllddddlllluuuuuulllluuuuurrrrddrduuulllrdldddduuruuldluuuurruuudddllllluuuurrdl";
    test_level_solution(8, inputs);
}

#[test]
fn test_level9() {
    let inputs = "rurrdlddrdllllllurrdluuuuurrrrrrr";
    test_level_solution(9, inputs);
}

#[test]
fn test_level10() {
    let inputs = "luurdduururdddlulluuurddrdllulddrurruurdldruullddlurl";
    test_level_solution(10, inputs);
}

#[test]
fn test_level11() {
    let inputs = "dlddruudrrrrruuuullluldllurulddrrurrrrrdldddddlulllllllldrrrululuuurrrrrrrrdddlldlllllllluuddrurrrdrrrruuuullluldlllururdrrrdldd";
    test_level_solution(11, inputs);
}

#[test]
fn test_level12() {
    let inputs = "rrruuurullulllrrrdldrruldurdllllulllllrrrrrulurdrrrulllldurdddrrullllrrdrrdddlldllddulllldrdduuluurrurruurrrrdludlluurrullddrrdludlulllllluulrrrrddddrrrdlluulrdullululurrrlrulldddrdluuluulurrlrdlddddluulurruuuurullllrrrdrruuuullrrddddllurdruuudlllllrrrulrddrlllllldrdrdrrddddllululluurrldrrludruruuudllurdrrlulurdruuudllllrdrrddddlllldludllurrrrruuuruldluuurrdlll";
    test_level_solution(12, inputs);
}

#[test]
fn test_level13() {
    let inputs = "ddduuurrdddlldruruuuurrruldddddddlluuldrdrruuuuuulldlluurddlddddrrdrruuuuururrdddllldddlluluuluuurdrurlluldddddrrddrruuuuururrdddllldddlluuuuu";
    test_level_solution(13, inputs);
}

#[test]
fn test_level14() {
    let inputs = "rddllrrrlldlddddrruldluulludrrurrurlurlldlulrruurruurrdudlluuuulddddrurdldddddrddlllurlldrrrruruulddrdllulrdllulrd";
    test_level_solution(14, inputs);
}

#[test]
fn test_level15() {
    let inputs = "rduuluurulllllllrrruulddlrddudlulllllldruurlld";
    test_level_solution(15, inputs);
}
