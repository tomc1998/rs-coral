mod layout;

use std::ops::Add;

pub use self::layout::Constraints;

/// A struct for an on-screen vector - size or position.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct ScreenVec {
    pub x: i32,
    pub y: i32,
}
impl ScreenVec {
    pub fn new(x: i32, y: i32) -> ScreenVec {
        ScreenVec { x, y }
    }
}
impl Add for ScreenVec {
    type Output = ScreenVec;
    fn add(self, rhs: ScreenVec) -> Self::Output {
        ScreenVec::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { a, r, g, b }
    }
}
