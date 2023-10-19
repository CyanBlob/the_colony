use bevy::app::App;
use bevy::prelude::*;
use bevy_enum_filter::{Enum, EnumFilter};
use rand::{Rng, thread_rng};
use crate::task_scorer::{Busy, Task};
use strum_macros::{EnumString, IntoStaticStr, AsRefStr};
use crate::AppState;
use crate::AppState::InGame;
use crate::character_plugin::Character;

#[derive(EnumFilter, Component, Debug, Clone, Copy, EnumString, IntoStaticStr, AsRefStr)]
pub enum AllTasks {
    Wander,
    Drink,
    Eat,
    Sleep,
}

impl Default for AllTasks {
    fn default() -> Self {
        AllTasks::Wander
    }
}

#[derive(Component)]
pub struct Thirst {
    pub value: f32,
    pub drain_rate: f32,
}

#[derive(Component)]
pub struct Hunger {
    pub value: f32,
    pub drain_rate: f32,
}

#[derive(Component)]
pub struct Sleep {
    pub value: f32,
    pub drain_rate: f32,
}

impl Default for Thirst {
    fn default() -> Self {
        Thirst {
            value: 100.0,
            drain_rate: 4.0,
        }
    }
}

impl Default for Hunger {
    fn default() -> Self {
        Hunger {
            value: 100.0,
            drain_rate: 2.0,
        }
    }
}

impl Default for Sleep {
    fn default() -> Self {
        Sleep {
            value: 100.0,
            drain_rate: 1.0,
        }
    }
}


impl Task for Thirst {
    fn score(&self) -> f32 {
        if self.value < 50.0 {
            11.0
        } else {
            0.0
        }
    }
}

impl Task for Hunger {
    fn score(&self) -> f32 {
        if self.value < 30.0 {
            10.0
        } else {
            0.0
        }
    }
}

impl Task for Sleep {
    fn score(&self) -> f32 {
        if self.value < 30.0 {
            9.0
        } else {
            0.0
        }
    }
}

fn hunger_system(time: Res<Time>, mut query: Query<&mut Hunger>) {
    for mut hunger in query.iter_mut() {
        hunger.value -= hunger.drain_rate * time.delta_seconds();
    }
}

fn thirst_system(time: Res<Time>, mut query: Query<&mut Thirst>) {
    for mut thirst in query.iter_mut() {
        thirst.value -= thirst.drain_rate * time.delta_seconds();
    }
}

fn sleep_system(time: Res<Time>, mut query: Query<&mut Sleep>) {
    for mut sleep in query.iter_mut() {
        sleep.value -= sleep.drain_rate * time.delta_seconds();
    }
}

fn eat(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Hunger, With<Character>, With<Enum!(AllTasks::Eat)>)>)
{
    for (entity, mut hunger, _, _) in query.iter_mut() {
        hunger.value += thread_rng().gen_range(10.0..50.0) * time.delta_seconds() + hunger.drain_rate * time.delta_seconds();
        if hunger.value >= 100.0 {
            commands.entity(entity).remove::<Busy>();
        }
    }
}

fn drink(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Thirst, With<Character>, With<Enum!(AllTasks::Drink)>)>)
{
    for (entity, mut thirst, _, _) in query.iter_mut() {
        thirst.value += thread_rng().gen_range(10.0..50.0) * time.delta_seconds() + thirst.drain_rate * time.delta_seconds();
        if thirst.value >= 100.0 {
            commands.entity(entity).remove::<Busy>();
        }
    }
}

fn sleep(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Sleep, With<Character>, With<Enum!(AllTasks::Sleep)>)>)
{
    for (entity, mut sleep, _, _) in query.iter_mut() {
        sleep.value += thread_rng().gen_range(2.0..16.0) * time.delta_seconds() + sleep.drain_rate * time.delta_seconds();
        if sleep.value >= 100.0 {
            commands.entity(entity).remove::<Busy>();
        }
    }
}

pub struct BasicTasksPlugin;

impl Plugin for BasicTasksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hunger_system.run_if(in_state(AppState::InGame)))
            .add_systems(Update, thirst_system.run_if(in_state(AppState::InGame)))
            .add_systems(Update, sleep_system.run_if(in_state(AppState::InGame)))
            .add_systems(Update, eat.run_if(in_state(InGame)))
            .add_systems(Update, drink.run_if(in_state(InGame)))
            .add_systems(Update, sleep.run_if(in_state(InGame)));
    }
}