use bevy::prelude::*;

use crate::primitives::*;

pub const ARENA_WIDTH: f32 = 51.0;
pub const ARENA_HEIGHT: f32 = ARENA_WIDTH;
pub const ARENA_BUFFER: f32 = 10.0;
pub const ARENA_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct Arena;

pub fn spawn_arena(mut commands: Commands) {
    commands.spawn().insert(Arena).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: ARENA_COLOR,
            ..Default::default()
        },
        ..Default::default()
    }).insert(Position { x: 0, y: 0 })
    .insert(Rec { width: ARENA_WIDTH as i32, height: ARENA_HEIGHT as i32 });
}