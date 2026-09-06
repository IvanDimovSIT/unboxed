use macroquad::input::{KeyCode, MouseButton, is_key_pressed, is_mouse_button_pressed};

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
