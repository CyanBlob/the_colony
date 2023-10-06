use crate::name_plugin::{Name, NeedsName};
use crate::{AppState, MyAssets};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use rand::{Rng, thread_rng};

#[derive(Component)]
pub struct Character;

#[derive(Component, Default)]
#[allow(unused)]
pub struct EntityTransform {
    position: Vec2,
    rotation: f32,
}

#[derive(Resource)]
struct TickTimer(Timer);

#[derive(Bundle)]
struct PlayerBundle {
    character: Character,
    transform: EntityTransform,
    sprite: SpriteBundle,
}

pub struct CharacterPlugin;

fn add_people(mut commands: Commands, assets: Res<MyAssets>) {
    let mut rand = thread_rng();

    for _ in 0..4 {
        commands.spawn((PlayerBundle {
            transform: Default::default(),
            sprite: SpriteBundle {
                texture: assets.character.clone(),
                transform: Transform::from_xyz(rand.gen_range(-320.0..320.0), rand.gen_range(-320.0..320.0), 100.0),
                ..default()
            },
            character: Character,
        }, NeedsName));
    }



    /*commands.spawn((Character, EntityTransform::default(), NeedsName));
    commands.spawn((Character, EntityTransform::default(), NeedsName));
    commands.spawn((
        Character,
        EntityTransform::default(),
        Name("Zayna Nieves".to_string()),
    ));*/
}

fn tick_pop(
    time: Res<Time>,
    mut timer: ResMut<TickTimer>,
    query: Query<(&mut EntityTransform, &Character, &Name)>,
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
            .add_systems(OnExit(AppState::Loading), add_people)
            .add_systems(Update, tick_pop.run_if(in_state(AppState::InGame)));
    }
}
