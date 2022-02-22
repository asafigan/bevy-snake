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
            .add_system(render_progress_bars)
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

#[derive(Component, Debug, Clone, Copy)]
pub struct Bar {
    pub bottom_left: Vec2,
    pub top_right: Vec2,
}

fn render_bar(bar: Bar, transform: &mut Transform) {
    let scale_x = bar.top_right.x - bar.bottom_left.x;
    let x = bar.bottom_left.x + (scale_x / 2.0);

    let scale_y = bar.top_right.y - bar.bottom_left.y;
    let y = bar.bottom_left.y + (scale_y / 2.0);

    transform.translation.x = x;
    transform.translation.y = y;

    transform.scale.x = scale_x;
    transform.scale.y = scale_y;
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ProgressBar {
    pub bar: Bar,
    pub percent: f32,
}

fn render_progress_bar(progress_bar: ProgressBar, transform: &mut Transform) {
    let bar = progress_bar.bar;
    let scale_x = bar.top_right.x - bar.bottom_left.x;
    let x = bar.bottom_left.x + (scale_x * progress_bar.percent);

    let bar = Bar {
        bottom_left: bar.bottom_left,
        top_right: Vec2::new(x, bar.top_right.y),
    };

    render_bar(bar, transform);
}

fn render_progress_bars(mut bars: Query<(&ProgressBar, &mut Transform), Changed<ProgressBar>>) {
    for (bar, mut transform) in bars.iter_mut() {
        render_progress_bar(*bar, &mut transform);
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
