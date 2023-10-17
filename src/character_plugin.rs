use crate::name_plugin::{Name, NeedsName};
use crate::{AppState, MyAssets};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::tasks::*;

#[derive(Component)]
pub struct Character;

#[derive(Resource)]
struct TickTimer(Timer);

#[derive(Bundle)]
struct PlayerBundle {
    character: Character,
    sprite: SpriteBundle,
    thirst: Thirst,
    hunger: Hunger,
    target_task: AllTasks,
}

pub struct CharacterPlugin;

fn add_people(mut commands: Commands, assets: Res<MyAssets>) {
    let mut rand = thread_rng();

    for _ in 0..10 {
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
                thirst: Thirst::default(),
                hunger: Hunger::default(),
                target_task: AllTasks::default(),
            },
            NeedsName,
            /*Text2dBundle {
                transform: Default::default(),
                text_anchor: Default::default(),
                text_2d_bounds: Default::default(),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
                text_layout_info: Default::default(),
            }*/
        )).with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section("TEST TEXT", TextStyle::default()),
                text_anchor: Default::default(),
                text_2d_bounds: Default::default(),
                transform: Transform::from_xyz(0.0, 20.0, 10.0),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
                text_layout_info: Default::default(),
            });
        });
    }
}

fn tick_pop(query: Query<(&Character, &Name), Changed<Name>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    for (_, name) in &query {
        //println!("Hello: {}!", name.0);
    }
}

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(OnExit(AppState::Loading), add_people)
            .add_systems(Update, tick_pop.run_if(in_state(AppState::InGame)));
    }
}
