use bevy::prelude::*;

mod arena;
mod game;
mod primitives;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(primitives::PrimitivesPlugin)
        .add_plugin(game::GamePlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
