use bevy::prelude::*;

mod arena;
mod game_loop;
mod game_over;
mod game_state;
mod pause_menu;
mod primitives;
mod start_menu;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(primitives::PrimitivesPlugin)
        .add_plugin(game_loop::GameLoopPlugin)
        .add_plugin(game_over::GameOverPlugin)
        .add_plugin(game_state::GameStatePlugin)
        .add_plugin(start_menu::StartMenuPlugin)
        .add_plugin(pause_menu::PauseMenuPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
