use std::ops::{Add, AddAssign, Neg, Sub};

use bevy::prelude::*;

use crate::arena::{ARENA_BUFFER, ARENA_HEIGHT, ARENA_WIDTH};

pub struct PrimitivesPlugin;

impl Plugin for PrimitivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_transform_with_changed_changed_scale)
            .add_system(update_transform_with_changed_position)
            .add_system(scale_positions)
            .add_system(scale_changed_positions)
            .add_system(update_scaling)
            .init_resource::<Scaling>();
    }
}

#[derive(Default)]
struct Scaling(f32);

#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        Position {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        self + -rhs
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Component)]
pub struct Rec {
    pub width: i32,
    pub height: i32,
}

impl Rec {
    pub fn contains(&self, position: Position) -> bool {
        let width = self.width as f32 / 2.0;
        let height = self.height as f32 / 2.0;
        let Position { x, y } = position;
        let x = x as f32;
        let y = y as f32;
        x < width && x > -width && y < height && y > -height
    }
}

fn update_scaling(mut scaling: ResMut<Scaling>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let scale = if window.height().partial_cmp(&window.width()) == Some(std::cmp::Ordering::Less) {
        window.height() / (ARENA_HEIGHT + ARENA_BUFFER)
    } else {
        window.width() / (ARENA_WIDTH + ARENA_BUFFER)
    };

    if scaling.0 != scale {
        scaling.0 = scale;
    }
}

fn scale_positions(scaling: Res<Scaling>, mut q: Query<(&Rec, &mut Transform)>) {
    if scaling.is_changed() {
        let scale = scaling.0;
        for (rec, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                rec.width as f32 * scale,
                rec.height as f32 * scale,
                transform.scale.z,
            );
        }
    }
}

fn scale_changed_positions(
    scaling: Res<Scaling>,
    mut q: Query<(&Rec, &mut Transform), Changed<Rec>>,
) {
    if !scaling.is_changed() {
        let scale = scaling.0;
        for (rec, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                rec.width as f32 * scale,
                rec.height as f32 * scale,
                transform.scale.z,
            );
        }
    }
}

fn update_transform_with_changed_position(
    scaling: Res<Scaling>,
    mut changed: Query<(&Position, &mut Transform), Changed<Position>>,
) {
    let scale = scaling.0;
    if !scaling.is_changed() {
        update_transforms_generic(scale, changed.iter_mut());
    }
}

fn update_transform_with_changed_changed_scale(
    scaling: Res<Scaling>,
    mut all: Query<(&Position, &mut Transform)>,
) {
    let scale = scaling.0;
    if scaling.is_changed() {
        update_transforms_generic(scale, all.iter_mut());
    }
}

fn update_transforms_generic<'a>(
    scale: f32,
    iter: impl Iterator<Item = (&'a Position, Mut<'a, Transform>)>,
) {
    for (position, mut transform) in iter {
        transform.translation.x = position.x as f32 * scale;
        transform.translation.y = position.y as f32 * scale;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        use self::Direction::*;
        match value {
            Up => Position { x: 0, y: 1 },
            Down => Position { x: 0, y: -1 },
            Left => Position { x: -1, y: 0 },
            Right => Position { x: 1, y: 0 },
        }
    }
}
