use macroquad::input::{KeyCode, MouseButton, is_key_pressed, is_mouse_button_pressed};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MovementInput {
    Left,
    Right,
    Down,
    Up,
    Undo,
    Reset,
}
impl std::fmt::Debug for MovementInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MovementInput::Left => "Left",
            MovementInput::Right => "Right",
            MovementInput::Down => "Down",
            MovementInput::Up => "Up",
            MovementInput::Undo => "Undo",
            MovementInput::Reset => "Reset",
        };
        write!(f, "MovementInput::{}", name)
    }
}

pub fn get_movement_input() -> Option<MovementInput> {
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

pub fn next_level() -> bool {
    is_key_pressed(KeyCode::Space)
        || is_key_pressed(KeyCode::Enter)
        || is_key_pressed(KeyCode::KpEnter)
}
