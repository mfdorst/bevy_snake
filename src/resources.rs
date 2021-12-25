use bevy::prelude::*;

pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
    pub tail_material: Handle<ColorMaterial>,
}

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum Snake {
    Input,
    Movement,
}
