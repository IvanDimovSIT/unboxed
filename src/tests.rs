use std::sync::LazyLock;

use crate::{
    input::MovementInput,
    level::Level,
    level_context::LevelContext,
    service::{level_loader, movement, win_condition::is_win},
};

static LEVELS: LazyLock<Vec<Level>> = LazyLock::new(level_loader::load_levels);

fn test_level_solution(level_index: usize, input_codes: &str) {
    let inputs = MovementInput::from_string(input_codes);
    let mut level_context = LevelContext::new(LEVELS[level_index].clone(), level_index);

    for input in inputs {
        assert!(
            !is_win(&level_context.level),
            "Level has been won earlier than expected"
        );
        movement::process(&mut level_context, Some(input));
    }
    assert!(is_win(&level_context.level), "Level has not been won");
}

#[test]
#[should_panic]
fn test_level0_losing() {
    let inputs = "rrruurdldrrrrdruu";
    test_level_solution(0, inputs);
}

#[test]
fn test_level0() {
    let inputs = "rrrurdldrrrrrdruuu";
    test_level_solution(0, inputs);
}

#[test]
fn test_level1() {
    let inputs = "ddrrrrrrdrruuudddllurdruudlllllllrrlldrrrrrrrdrruuudddllurdruu";
    test_level_solution(1, inputs);
}

#[test]
fn test_level2() {
    let inputs = "rrrrrrrdrdruluurdddllurdruu";
    test_level_solution(2, inputs);
}

#[test]
fn test_level3() {
    let inputs = "llrrrrldluudldluruldlururrddrdllllrurdllluurrrrrr";
    test_level_solution(3, inputs);
}

#[test]
fn test_level4() {
    let inputs = "llullddldrrluurrrdrrdddddllllrdulldluururrrur";
    test_level_solution(4, inputs);
}

#[test]
fn test_level5() {
    let inputs = "ullllrurlurlruurrrulllluurrrurrulllrrurrrddlddddldddrrrlrurrurrddddrrr";
    test_level_solution(5, inputs);
}

#[test]
fn test_level6() {
    let inputs = "rruuurrrrrddddrurllduuurrulllldlllrullllrlldlddddurldrurdurdlldrrrlulururuullururdllllrldddurdrrdrrrrdldruuuurrrrrddddrru";
    test_level_solution(6, inputs);
}

#[test]
fn test_level7() {
    let inputs = "llllllllluuuulurrdddddldddddrurrrrudlllluuuuuuddrruulrulurrddruluurrdlllllrrrddllluluuuuuurrrrddrdlldddrrdrddrrrrurrrrdlllllllluuuuuulluruldlurdrruruldrdddddrrrrrrurullllllrrrddlllluuuuuulluuuurdrrrddrdlldddrrrrrrrruluurrdlldllluuurrrdlllllldrdrdrdlldllllluuuuuddddrdrrrddrdrrrrrrrrrdllllllullllddddlllluuuuuulllluuuuurrrrddrduuulllrdldddduuruuldluuuurruuudddllllluuuurrdl";
    test_level_solution(7, inputs);
}
