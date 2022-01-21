mod components;
mod consts;
mod food;
mod resources;
mod snake;

use bevy::prelude::*;
use components::{Position, Size};
use consts::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_owned(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(setup_background_color)
        .add_plugin(snake::SnakePlugin)
        .add_plugin(food::FoodPlugin)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(size_scaling)
                .with_system(position_translation),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_background_color(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: CLEAR_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Size::square(ARENA_HEIGHT as f32));
}

fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (size, mut sprite) in query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(
            size.width / ARENA_WIDTH as f32 * window.width() as f32,
            size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        ));
    }
}

fn position_translation(windows: Res<Windows>, mut query: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, window_bound: f32, game_bound: f32) -> f32 {
        let tile_size = window_bound / game_bound;
        pos / game_bound * window_bound - (window_bound / 2.0) + (tile_size / 2.0)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut xform) in query.iter_mut() {
        xform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
