use crate::name_plugin::{Name, NeedsName};
use crate::{AppState};
use bevy::app::{App, Plugin};
use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component, Default)]
#[allow(unused)]
pub struct Transform {
    position: Vec2,
    rotation: f32,
}

#[derive(Resource)]
struct TickTimer(Timer);

pub struct CharacterPlugin;

fn add_people(mut commands: Commands) {
    commands.spawn((Character, Transform::default(), NeedsName));
    commands.spawn((Character, Transform::default(), NeedsName));
    commands.spawn((
        Character,
        Transform::default(),
        Name("Zayna Nieves".to_string()),
    ));
}

fn tick_pop(
    time: Res<Time>,
    mut timer: ResMut<TickTimer>,
    query: Query<(&mut Transform, &Character, &Name)>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for (_, _, name) in &query {
            println!("Hello: {}!", name.0);
        }
    }
}

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, tick_pop.run_if(in_state(AppState::InGame)));
    }
}
