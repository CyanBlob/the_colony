use crate::character_plugin::Character;
use crate::AppState;
use bevy::app::App;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
use bevy_enum_filter::prelude::*;
use rand::{thread_rng, Rng};
use crate::AppState::Loading;
use crate::tasks::*;

pub struct RandomMovementPlugin;

#[derive(Resource)]
struct ChangeDirTimer(Timer);

#[derive(Component)]
pub struct Wandering;

#[derive(Component, Default)]
pub struct RandomDirection {
    dir: Vec2,
}

fn wander(
    mut commands: Commands,
    query: Query<Entity, (With::<Character>, With::<Enum!(AllTasks::Wander)>, Without::<Wandering>)>)
{
    for (entity) in &query {
        commands
            .entity(entity)
            .insert((Wandering, RandomDirection::default()));
    }
}

fn move_randomly(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &RandomDirection), (With<Wandering>, With<Enum!(AllTasks::Wander)>)>) {

    for (mut transform, dir) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * dir.dir.x * 5.0;
        transform.translation.y += time.delta_seconds() * dir.dir.y * 5.0;
    }
}

fn update_random_dir(
    time: Res<Time>,
    mut timer: ResMut<ChangeDirTimer>,
    mut query: Query<&mut RandomDirection, (With<Wandering>, With<Enum!(AllTasks::Wander)>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut dir in query.iter_mut() {
            update_random_dir_base(dir);
        }
    }
}

fn update_random_dir_without_tick(
    mut query: Query<&mut RandomDirection>,
) {
    for dir in query.iter_mut() {
        update_random_dir_base(dir);
    }
}

fn update_random_dir_base(mut dir: Mut<RandomDirection>) {
    dir.dir.x = thread_rng().gen_range(-10.0..10.0);
    dir.dir.y = thread_rng().gen_range(-10.0..10.0);
}

impl Plugin for RandomMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChangeDirTimer(Timer::from_seconds(2.0, Repeating)))
            .add_systems(OnExit(Loading), update_random_dir_without_tick)
            .add_systems(
                Update,
                wander.run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, move_randomly.run_if(in_state(AppState::InGame)))
            .add_systems(Update, update_random_dir.run_if(in_state(AppState::InGame)));
    }
}
