#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(dim: f32) -> Self {
        Self {
            width: dim,
            height: dim,
        }
    }
}
