use bevy::prelude::*;

use crate::primitives::ProgressBar;

#[derive(Debug, Default)]
pub struct Experience(pub usize);

#[derive(Component)]
pub struct ExperienceBar;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Experience>()
            .add_system(update_experience_bars)
            .add_system(init_experience_bars);
    }
}

fn update_experience_bars(
    experience: Res<Experience>,
    mut bars: Query<&mut ProgressBar, With<ExperienceBar>>,
) {
    if experience.is_changed() {
        for mut bar in bars.iter_mut() {
            bar.percent = experience.0 as f32 / 100.0;
        }
    }
}

fn init_experience_bars(
    experience: Res<Experience>,
    mut bars: Query<&mut ProgressBar, Added<ExperienceBar>>,
) {
    if !experience.is_changed() {
        for mut bar in bars.iter_mut() {
            bar.percent = experience.0 as f32 / 100.0;
        }
    }
}
