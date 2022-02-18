use bevy::prelude::*;

use crate::game_state::{CleanUp, GameState};

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::StartMenu).with_system(spawn_ui))
            .add_system_set(SystemSet::on_update(GameState::StartMenu).with_system(start_game));
    }
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("Start", text_style, text_alignment),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CleanUp::new(GameState::StartMenu));
}

fn start_game(input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Return) {
        state.replace(GameState::MainGameLoop).unwrap();
    }
}
