use std::fmt::Debug;
use std::hash::Hash;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    GameOver,
    MainGameLoop,
    StartMenu,
    PauseMenu,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        use GameState::*;

        app.add_state(StartMenu);

        let states = [GameOver, MainGameLoop, StartMenu, PauseMenu];

        for state in states {
            app.add_system_set(SystemSet::on_exit(state).with_system(clean_up(state)));
        }
    }
}

#[derive(Component)]
pub struct CleanUp<T> {
    state: T,
}

impl<T> CleanUp<T>
where
    T: Send + Sync + 'static + Clone + Hash + Debug + Eq,
{
    pub fn new(state: T) -> Self {
        CleanUp { state }
    }
}

fn clean_up<T>(state: T) -> impl Fn(Commands, Query<(Entity, &CleanUp<T>)>)
where
    T: Send + Sync + 'static + Clone + Hash + Debug + Eq,
{
    move |mut commands, entities| {
        for (entity, x) in entities.iter() {
            if state == x.state {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
