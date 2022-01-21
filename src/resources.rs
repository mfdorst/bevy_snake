use crate::components::Direction;
use bevy::prelude::*;
use std::collections::VecDeque;

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
