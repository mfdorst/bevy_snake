use crate::{
    components::{Food, Position, Size},
    consts::*,
};
use bevy::{core::FixedTimestep, prelude::*};
use rand::Rng;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FOOD_RESPAWN_TIME))
                .with_system(spawn_food),
        );
    }
}

fn spawn_food(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: rng.gen_range(0..ARENA_WIDTH) as i32,
            y: rng.gen_range(0..ARENA_HEIGHT) as i32,
        })
        .insert(Size::square(FOOD_SIZE));
}
