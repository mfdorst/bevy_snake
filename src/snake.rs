use bevy::prelude::*;

use crate::{
    components::{Position, Size},
    consts::*,
    resources::{Materials, SnakeHead},
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("spawn_snake", SystemStage::single(spawn_snake.system()))
            .add_system(snake_move.system());
    }
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(SnakeHead)
        .insert(Position::new(STARTING_POSITION_X, STARTING_POSITION_Y))
        .insert(Size::square(SNAKE_HEAD_SIZE));
}

fn snake_move(key: Res<Input<KeyCode>>, mut query: Query<(&SnakeHead, &mut Position)>) {
    for (_, mut pos) in query.iter_mut() {
        if key.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if key.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if key.pressed(KeyCode::Up) {
            pos.y += 1;
        }
        if key.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
    }
}
