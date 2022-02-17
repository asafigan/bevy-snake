
use std::hash::Hash;
use std::fmt::Debug;

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    GameOver,
    MainGameLoop,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::MainGameLoop)
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver)
                    .with_system(clean_up(GameState::GameOver))
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainGameLoop)
                    .with_system(clean_up(GameState::MainGameLoop))
            );
    }
}

#[derive(Component)]
pub struct CleanUp<T> {
    state: T,
}

impl<T> CleanUp<T> where T: Send + Sync + 'static + Clone + Hash + Debug + Eq {
    pub fn new(state: T) -> Self {
        CleanUp {
            state
        }
    }
}

fn clean_up<T>(state: T) -> impl Fn(Commands, Query<(Entity, &CleanUp<T>)>)
where T: Send + Sync + 'static + Clone + Hash + Debug + Eq
{
    move |mut commands, entities| {
        for (entity, x) in entities.iter() {
            if  state == x.state {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}


