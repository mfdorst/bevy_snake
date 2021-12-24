use bevy::prelude::*;

use crate::resources::{Materials, SnakeHead};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("spawn_snake", SystemStage::single(spawn_snake.system()))
            .add_system(snake_move.system());
    }
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        ..Default::default()
    });
}

fn snake_move(mut query: Query<(&SnakeHead, &mut Transform)>) {
    for (_, mut xform) in query.iter_mut() {
        xform.translation.y += 1.0;
    }
}
