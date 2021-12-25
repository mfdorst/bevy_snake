mod components;
mod consts;
mod food;
mod resources;
mod snake;

use bevy::prelude::*;
use components::{Position, Size};
use consts::*;
use resources::Materials;

fn main() {
    let mut app = App::build();
    app.insert_resource(WindowDescriptor {
        title: "Snake!".to_owned(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    })
    .insert_resource(ClearColor(CLEAR_COLOR))
    .add_startup_system(setup.system())
    .add_plugin(snake::SnakePlugin)
    .add_plugin(food::FoodPlugin)
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(size_scaling.system())
            .with_system(position_translation.system()),
    )
    .add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.run();
}

fn setup(mut commands: Commands, mut color_materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let head_material = color_materials.add(HEAD_COLOR.into());
    let food_material = color_materials.add(FOOD_COLOR.into());
    commands.insert_resource(Materials {
        head_material,
        food_material,
    })
}

fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (size, mut sprite) in query.iter_mut() {
        sprite.size = Vec2::new(
            size.width / ARENA_WIDTH as f32 * window.width() as f32,
            size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
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
