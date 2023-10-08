use bevy::app::App;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
use rand::{Rng, thread_rng};
use crate::AppState;
use crate::character_plugin::Character;

pub struct RandomMovementPlugin;

#[derive(Component)]
pub struct RandomMovement;

#[derive(Resource)]
struct ChangeDirTimer(Timer);

#[derive(Component, Default)]
pub struct RandomDirection {
    dir: Vec2,
}

fn add_random_movement(
    mut commands: Commands,
    query: Query<(Entity, With<Character>), Without<RandomMovement>>,
) {
    for (entity, _) in &query {
        commands.entity(entity).insert((RandomMovement, RandomDirection::default()));
    }
}

fn move_randomly(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &RandomDirection), With<RandomMovement>>,
) {
    for (mut transform, dir) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * dir.dir.x;
        transform.translation.y += time.delta_seconds() * dir.dir.y;
    }
}

fn update_random_dir(
    time: Res<Time>,
    mut timer: ResMut<ChangeDirTimer>,
    mut query: Query<(&mut RandomDirection, With<RandomMovement>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut dir in query.iter_mut() {
            dir.0.dir.x = thread_rng().gen_range(-10.0..10.0);
            dir.0.dir.y = thread_rng().gen_range(-10.0..10.0);
        }
    }
}

impl Plugin for RandomMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChangeDirTimer(Timer::from_seconds(2.0, Repeating)))
            .add_systems(Update, add_random_movement.run_if(in_state(AppState::InGame)))
            .add_systems(Update, move_randomly.run_if(in_state(AppState::InGame)))
            .add_systems(Update, update_random_dir.run_if(in_state(AppState::InGame)));
    }
}