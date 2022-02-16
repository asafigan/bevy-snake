use std::collections::VecDeque;

use bevy::{prelude::*, core::FixedTimestep};
use rand::Rng;

use crate::primitives::*;
use crate::arena::{ARENA_HEIGHT, ARENA_WIDTH, Arena};
use crate::primitives::Direction;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_snake)
        .add_startup_system(crate::arena::spawn_arena)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::steps_per_second(20.0))
            .with_system(move_snake_head.chain(move_snake_tail))
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.2))
            .with_system(spawn_apple)
        )
        .add_system(snake_controls)
        .add_system(kill_snake_outside_arena)
        .add_system(kill_snake_hitting_tail)
        .add_system(eat_food);
    }
}

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.0, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(0.0, 0.7, 0.0);

#[derive(Component, Default)]
struct SnakeHead {
    direction: Direction,
    heading: Direction,
    tail: VecDeque<Entity>,
    length: usize,
    last_position: Position,
}

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Tail;

fn spawn_snake(mut commands: Commands) {
    commands.spawn().insert(SnakeHead::default()).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_HEAD_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Position { x: 0, y: 0 })
    .insert(Rec { width: 1, height: 1 });
}

fn move_snake_head(mut heads: Query<(&mut SnakeHead, &mut Position)>) {
    for (mut head, mut position) in heads.iter_mut() {
        head.last_position = *position;
        *position += head.heading * 1;
        head.direction = head.heading;
    }
}

fn move_snake_tail(mut commands: Commands, mut heads: Query<&mut SnakeHead>, mut positions: Query<&mut Position>) {
    for mut head in heads.iter_mut() {
        let mut last_position = head.last_position;
        if let Some(entity) = head.tail.pop_front() {
            let mut position = positions.get_mut(entity).unwrap();
            head.tail.push_back(entity);
            std::mem::swap(position.as_mut(), &mut last_position);
        }

        if head.length > head.tail.len(){
            let entity = commands.spawn().insert(Tail).insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(last_position)
            .insert(Rec { width: 1, height: 1 })
            .id();

            head.tail.push_front(entity);
        }
    }
}

fn spawn_apple(mut commands: Commands) {
    let width = ARENA_WIDTH / 2 as f32;
    let height = ARENA_HEIGHT / 2 as f32;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: rand::thread_rng().gen_range(-width..width) as i32,
            y: rand::thread_rng().gen_range(-height..height) as i32,
        })
        .insert(Rec { width: 1, height: 1});
}

fn eat_food(
    mut commands: Commands, 
    food: Query<(Entity, &Position), With<Food>>,
    mut heads: Query<(&Position, &mut SnakeHead), With<SnakeHead>>
) {
    for (entity, food) in food.iter() {
        for (position, mut head) in heads.iter_mut() {
            if food == position {
                commands.entity(entity).despawn();
                head.length += 1;
            }
        }
    }
}

fn snake_controls(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut SnakeHead>) {
    for mut head in q.iter_mut() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.heading = dir;
        }
    }
}

fn kill_snake_outside_arena(mut commands: Commands, snakes: Query<(Entity, &Position), With<SnakeHead>>, arenas: Query<(&Position, &Rec), With<Arena>>) {
    for (entity, snake) in snakes.iter() {
        if arenas.iter().all(|(position, rec)| !rec.contains(*position + *snake)) {
            commands.entity(entity).despawn();
        }
    }
}

fn kill_snake_hitting_tail(mut commands: Commands, snakes: Query<(Entity, &Position), With<SnakeHead>>, tails: Query<&Position, With<Tail>>) {
    for (entity, snake) in snakes.iter() {
        if tails.iter().any(|position| position == snake) {
            commands.entity(entity).despawn();
        }
    }
}