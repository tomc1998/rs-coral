mod layout;

pub use self::layout::Constraints;

/// A struct for an on-screen vector - size or position.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct ScreenVec {
    pub x: i32,
    pub y: i32,
}
