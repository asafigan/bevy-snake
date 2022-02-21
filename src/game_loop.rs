use std::collections::VecDeque;
use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::arena::*;
use crate::game_state::{CleanUp, GameState};
use crate::primitives::Direction;
use crate::primitives::*;

pub struct GameLoopPlugin;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, SystemLabel)]
pub enum GameStage {
    Movement,
    Collection,
}

#[derive(Debug, Default)]
struct Score(usize);

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        use GameStage::*;

        app.add_event::<CollectEvent>()
            .init_resource::<Score>()
            .add_system_set(
                SystemSet::on_enter(GameState::MainGameLoop)
                    .with_system(spawn_snake)
                    .with_system(spawn_arena)
                    .with_system(spawn_score_board)
                    .with_system(reset_score),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGameLoop)
                    .with_system(move_snake_head.chain(move_snake_tail).label(Movement))
                    .with_system(spawn_apple)
                    .with_system(snake_controls)
                    .with_system(kill_snake_outside_arena)
                    .with_system(kill_snake_hitting_tail)
                    .with_system(update_score_board)
                    .with_system(collect_food.after(Movement).label(Collection))
                    .with_system(despawn_food.after(Collection))
                    .with_system(grow_snake.after(Collection))
                    .with_system(track_score.after(Collection))
                    .with_system(pause_game.chain(game_over)),
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

#[derive(Component)]
struct ScoreBoard;

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

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

fn spawn_score_board(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            text: Text::with_section("0", text_style, text_alignment),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreBoard)
        .insert(Position { x: 0, y: 0 })
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

struct CollectEvent {
    food: Entity,
    snake: Entity,
}

fn collect_food(
    mut events: EventWriter<CollectEvent>,
    food: Query<(Entity, &Position), With<Food>>,
    mut heads: Query<(Entity, &Position), With<SnakeHead>>,
) {
    for (food, a) in food.iter() {
        for (snake, b) in heads.iter_mut() {
            if a == b {
                events.send(CollectEvent { food, snake });
            }
        }
    }
}

fn despawn_food(mut commands: Commands, mut events: EventReader<CollectEvent>) {
    for event in events.iter() {
        commands.entity(event.food).despawn();
    }
}

fn grow_snake(mut events: EventReader<CollectEvent>, mut heads: Query<&mut SnakeHead>) {
    for event in events.iter() {
        if let Ok(mut snake) = heads.get_mut(event.snake) {
            snake.length += 1;
        }
    }
}

fn track_score(mut events: EventReader<CollectEvent>, mut score: ResMut<Score>) {
    score.0 += events.iter().count();
}

fn update_score_board(score: Res<Score>, mut query: Query<&mut Text, With<ScoreBoard>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = score.0.to_string();
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
        app_state.overwrite_push(GameState::GameOver).unwrap();
    }
}

fn pause_game(mut app_state: ResMut<State<GameState>>, inputs: Res<Input<KeyCode>>) {
    if inputs.just_pressed(KeyCode::P) {
        app_state.push(GameState::PauseMenu).unwrap();
    }
}
