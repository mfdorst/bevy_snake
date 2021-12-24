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
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(SnakeHead);
}

fn snake_move(key: Res<Input<KeyCode>>, mut query: Query<(&SnakeHead, &mut Transform)>) {
    for (_, mut xform) in query.iter_mut() {
        let left = key.pressed(KeyCode::Left);
        let right = key.pressed(KeyCode::Right);
        let up = key.pressed(KeyCode::Up);
        let down = key.pressed(KeyCode::Down);
        let x = if left && !right {
            -2.0
        } else if right && !left {
            2.0
        } else {
            0.0
        };
        let y = if down && !up {
            -2.0
        } else if up && !down {
            2.0
        } else {
            0.0
        };
        xform.translation.x += x;
        xform.translation.y += y;
    }
}
