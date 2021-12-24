mod resources;
mod snake;

use bevy::prelude::*;
use resources::Materials;
use snake::SnakePlugin;

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_startup_system(setup.system())
        .add_plugin(SnakePlugin)
        .run();
}

fn setup(mut commands: Commands, mut color_materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: color_materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    })
}
