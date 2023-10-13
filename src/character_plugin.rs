use crate::name_plugin::{Name, NeedsName};
use crate::{AppState, MyAssets};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use big_brain::prelude::{FirstToScore, Thinker};
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Thirst {

}

#[derive(Resource)]
struct TickTimer(Timer);

#[derive(Bundle)]
struct PlayerBundle {
    character: Character,
    sprite: SpriteBundle,
    //thirst: Thirst,
}

pub struct CharacterPlugin;

fn add_people(mut commands: Commands, assets: Res<MyAssets>) {
    let mut rand = thread_rng();

    for _ in 0..1 {
        commands.spawn((
            PlayerBundle {
                sprite: SpriteBundle {
                    texture: assets.character.clone(),
                    transform: Transform::from_xyz(
                        rand.gen_range(-1000.0..1000.0),
                        rand.gen_range(-640.0..640.0),
                        100.0,
                    ),
                    ..default()
                },
                character: Character,
                /*thirst: Thirst {
                    thirst: rand.gen_range(75.0..76.0),
                    per_second: rand.gen_range(1.5..2.5),
                },*/
            },
            NeedsName,
        ));
    }
}

fn tick_pop(query: Query<(&Character, &Name), Changed<Name>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    for (_, name) in &query {
        println!("Hello: {}!", name.0);
    }
}

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(OnExit(AppState::Loading), add_people)
            .add_systems(Update, tick_pop.run_if(in_state(AppState::InGame)));
    }
}
