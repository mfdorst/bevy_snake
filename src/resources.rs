use crate::components::Direction;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
    pub tail_material: Handle<ColorMaterial>,
}

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum SnakeSystem {
    Input,
    Movement,
}

pub struct Snake {
    pub head: Entity,
    pub tail: VecDeque<Entity>,
}

pub struct InputDirection(pub Direction);
pub struct CurrentDirection(pub Direction);
