use bevy::prelude::*;

use crate::{game_state::{GameState, CleanUp}, primitives::Rec};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(spawn_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(restart_game)
        );
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
            text: Text::with_section("Game Over", text_style.clone(), text_alignment),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CleanUp::new(GameState::GameOver));
    
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 0.5),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Rec{width: 10000, height: 10000})
        .insert(CleanUp::new(GameState::GameOver));
}

fn restart_game(input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Return) {
        state.replace(GameState::MainGameLoop).unwrap();
    }
}