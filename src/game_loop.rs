use std::collections::VecDeque;
use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::arena::*;
use crate::game_state::{GameState, CleanUp};
use crate::primitives::Direction;
use crate::primitives::*;

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainGameLoop)
                .with_system(spawn_snake)
                .with_system(spawn_arena),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainGameLoop)
                .with_system(move_snake_head.chain(move_snake_tail))
                .with_system(spawn_apple)
                .with_system(game_over)
                .with_system(snake_controls)
                .with_system(kill_snake_outside_arena)
                .with_system(kill_snake_hitting_tail)
                .with_system(eat_food)
        );
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
    dead: bool,
}

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Tail;

pub fn spawn_arena(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: ARENA_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Arena)
        .insert(Position { x: 0, y: 0 })
        .insert(Rec {
            width: ARENA_WIDTH as i32,
            height: ARENA_HEIGHT as i32,
        })
        .insert(CleanUp::new(GameState::MainGameLoop));
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead::default())
        .insert(Timer::new(Duration::from_millis(100), true))
        .insert(Position { x: 0, y: 0 })
        .insert(Rec {
            width: 1,
            height: 1,
        })
        .insert(CleanUp::new(GameState::MainGameLoop));
}

fn move_snake_head(time: Res<Time>, mut heads: Query<(&mut SnakeHead, &mut Position, &mut Timer)>) {
    for (mut head, mut position, mut timer) in heads.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            head.last_position = *position;
            *position += head.heading.into();
            head.direction = head.heading;
        }
    }
}

fn move_snake_tail(
    mut commands: Commands,
    mut heads: Query<(&mut SnakeHead, &Timer)>,
    mut positions: Query<&mut Position>,
) {
    for (mut head, timer) in heads.iter_mut() {
        if timer.just_finished() {
            let mut last_position = head.last_position;
            if let Some(entity) = head.tail.pop_front() {
                let mut position = positions.get_mut(entity).unwrap();
                head.tail.push_back(entity);
                std::mem::swap(position.as_mut(), &mut last_position);
            }
    
            if head.length > head.tail.len() {
                let entity = commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: SNAKE_HEAD_COLOR,
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Tail)
                    .insert(last_position)
                    .insert(Rec {
                        width: 1,
                        height: 1,
                    })
                    .insert(CleanUp::new(GameState::MainGameLoop))
                    .id();
    
                head.tail.push_front(entity);
            }
        }
    }
}

fn spawn_apple(mut commands: Commands, food: Query<&Food>) {
    if food.iter().count() < 1 {
        let width = ARENA_WIDTH / 2.0;
        let height = ARENA_HEIGHT / 2.0;
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.5),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Food)
            .insert(Position {
                x: rand::thread_rng().gen_range(-width..width) as i32,
                y: rand::thread_rng().gen_range(-height..height) as i32,
            })
            .insert(Rec {
                width: 1,
                height: 1,
            })
            .insert(CleanUp::new(GameState::MainGameLoop));
    }
}

fn eat_food(
    mut commands: Commands,
    food: Query<(Entity, &Position), With<Food>>,
    mut heads: Query<(&Position, &mut SnakeHead), With<SnakeHead>>,
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

fn kill_snake_outside_arena(
    mut snakes: Query<(&Position, &mut SnakeHead), With<SnakeHead>>,
    arenas: Query<(&Position, &Rec), With<Arena>>,
) {
    for (snake, mut head) in snakes.iter_mut() {
        if arenas
            .iter()
            .all(|(position, rec)| !rec.contains(*position + *snake))
        {
            head.dead = true;
        }
    }
}

fn kill_snake_hitting_tail(
    mut snakes: Query<(&Position, &mut SnakeHead)>,
    tails: Query<&Position, With<Tail>>,
) {
    for (snake, mut head) in snakes.iter_mut() {
        if tails.iter().any(|position| position == snake) {
            head.dead = true;
        }
    }
}

fn game_over(mut app_state: ResMut<State<GameState>>, snakes: Query<&SnakeHead>) {
    if snakes.iter().all(|x| x.dead) {
        app_state.push(GameState::GameOver).unwrap();
    }
}
