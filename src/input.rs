use macroquad::{
    input::{
        KeyCode, MouseButton, is_key_pressed, is_mouse_button_pressed, is_mouse_button_released,
        mouse_position,
    },
    math::{Vec2, vec2},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementInput {
    Left,
    Right,
    Down,
    Up,
    Undo,
    Reset,
}
impl MovementInput {
    #[cfg(debug_assertions)]
    pub fn slice_to_string(slice: &[MovementInput]) -> String {
        slice.iter().copied().map(Self::to_code).collect()
    }

    #[cfg(test)]
    pub fn from_string(string: &str) -> Vec<MovementInput> {
        string.chars().map(Self::from_code).collect()
    }

    fn to_code(self) -> char {
        match self {
            MovementInput::Left => 'l',
            MovementInput::Right => 'r',
            MovementInput::Down => 'd',
            MovementInput::Up => 'u',
            MovementInput::Undo => 'z',
            MovementInput::Reset => 'R',
        }
    }

    #[cfg(test)]
    fn from_code(code: char) -> MovementInput {
        match code {
            'l' => MovementInput::Left,
            'r' => MovementInput::Right,
            'u' => MovementInput::Up,
            'd' => MovementInput::Down,
            'z' => MovementInput::Undo,
            'R' => MovementInput::Reset,
            _ => panic!("Unrecognised code"),
        }
    }
}

pub fn get_movement_input_desktop() -> Option<MovementInput> {
    if left() {
        Some(MovementInput::Left)
    } else if right() {
        Some(MovementInput::Right)
    } else if up() {
        Some(MovementInput::Up)
    } else if down() {
        Some(MovementInput::Down)
    } else if undo() {
        Some(MovementInput::Undo)
    } else if reset() {
        Some(MovementInput::Reset)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct MobileInputReader {
    start: Option<Vec2>,
}
impl MobileInputReader {
    pub fn new() -> Self {
        Self { start: None }
    }

    pub fn reset(&mut self) {
        self.start = None;
    }

    pub fn get_movement_input_mobile(&mut self, mouse_vec: Vec2) -> Option<MovementInput> {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.start = Some(mouse_vec);
            None
        } else if is_mouse_button_released(MouseButton::Left) {
            if let Some(start_p) = self.start {
                let direction = mouse_vec - start_p;
                Some(Self::determine_movement_type(direction))
            } else {
                self.start = None;
                None
            }
        } else {
            None
        }
    }

    fn determine_movement_type(direction: Vec2) -> MovementInput {
        if direction.x.abs() > direction.y.abs() {
            if direction.x < 0.0 {
                MovementInput::Left
            } else {
                MovementInput::Right
            }
        } else {
            if direction.y < 0.0 {
                MovementInput::Up
            } else {
                MovementInput::Down
            }
        }
    }
}

pub fn left() -> bool {
    is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left)
}

pub fn right() -> bool {
    is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right)
}

pub fn up() -> bool {
    is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up)
}

pub fn down() -> bool {
    is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down)
}

pub fn reset() -> bool {
    is_key_pressed(KeyCode::R)
}

pub fn undo() -> bool {
    is_key_pressed(KeyCode::Z)
}

pub fn exit() -> bool {
    is_key_pressed(KeyCode::Escape)
}

pub fn click() -> bool {
    is_mouse_button_pressed(MouseButton::Left)
}

pub fn next_level(is_mobile: bool) -> bool {
    if is_mobile {
        is_mouse_button_pressed(MouseButton::Left)
    } else {
        is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::KpEnter)
    }
}

pub fn get_mouse_vec() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}
