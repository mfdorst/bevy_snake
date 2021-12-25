use bevy::prelude::*;

pub struct SnakeHead;
pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
}
