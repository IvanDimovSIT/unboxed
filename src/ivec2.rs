use std::ops::{Add, AddAssign, Sub};

use macroquad::math::{Vec2, vec2};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Ivec2 {
    pub x: i32,
    pub y: i32,
}
impl Ivec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl Add for Ivec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Ivec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Into<Vec2> for Ivec2 {
    fn into(self) -> Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
}
impl AddAssign<Ivec2> for Ivec2 {
    fn add_assign(&mut self, rhs: Ivec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
