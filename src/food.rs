use crate::{
    components::{Food, Position, Size},
    consts::*,
    resources::Materials,
};
use bevy::{core::FixedTimestep, prelude::*};
use rand::Rng;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FOOD_RESPAWN_TIME))
                .with_system(spawn_food.system()),
        );
    }
}

fn spawn_food(mut commands: Commands, materials: Res<Materials>) {
    let mut rng = rand::thread_rng();
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.food_material.clone(),
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: rng.gen_range(0..ARENA_WIDTH) as i32,
            y: rng.gen_range(0..ARENA_HEIGHT) as i32,
        })
        .insert(Size::square(FOOD_SIZE));
}
