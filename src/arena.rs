use bevy::prelude::*;

pub const ARENA_WIDTH: f32 = 21.0;
pub const ARENA_HEIGHT: f32 = ARENA_WIDTH;
pub const ARENA_BUFFER: f32 = 4.0;
pub const ARENA_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct Arena;
