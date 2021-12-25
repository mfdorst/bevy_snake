use crate::components::Direction;
use bevy::prelude::Color;

// Dimensions
pub const ARENA_HEIGHT: u32 = 20;
pub const ARENA_WIDTH: u32 = 20;
pub const FOOD_SIZE: f32 = 0.8;
pub const SNAKE_HEAD_SIZE: f32 = 0.9;
pub const SNAKE_TAIL_SIZE: f32 = 0.7;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const WINDOW_WIDTH: f32 = 800.0;

// Positions
pub const STARTING_POSITION_X: i32 = 10;
pub const STARTING_POSITION_Y: i32 = 10;

// Timing
pub const FOOD_RESPAWN_TIME: f64 = 2.0;

// Colors
pub const CLEAR_COLOR: Color = Color::BLACK;
pub const FOOD_COLOR: Color = Color::rgb(0.4, 0.1, 0.6);
pub const HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const TAIL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

// Other
pub const INITIAL_SNAKE_LENGTH: i32 = 4;
pub const INITIAL_DIRECTION: Direction = Direction::Right;
