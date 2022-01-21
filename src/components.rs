use bevy::prelude::*;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
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

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct TailSegment;

#[derive(PartialEq, Copy, Clone, Component)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
