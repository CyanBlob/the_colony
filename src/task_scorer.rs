use bevy::prelude::*;
use bevy_enum_filter::Enum;
use rand::{Rng, thread_rng};
use crate::AppState::InGame;
use crate::character_plugin::Character;
use crate::tasks::*;

pub trait Task {
    fn score(&self) -> f32;
}

pub struct TaskScoringPlugin;

#[derive(Component)]
pub struct Busy;

fn score_basic_tasks(time: Res<Time>, mut query: Query<(Entity, &mut AllTasks, &mut Thirst, &mut Hunger), (Without<Busy>)>) {
    for (entity, mut task, thirst, mut hunger) in query.iter_mut() {
        let mut ratings = vec![(AllTasks::Wander, 1.0)];
        if hunger.value > 0.0 {
            hunger.value -= hunger.drain_rate * time.delta_seconds();
        }

        ratings.push((AllTasks::Eat, hunger.score()));
        ratings.push((AllTasks::Drink, thirst.score()));

        ratings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        *task = ratings.iter().nth(0).unwrap().0;
    }
}

fn eat(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Hunger, With<Character>, With<Enum!(AllTasks::Eat)>)>)
{
    for (entity, mut hunger, _, _) in query.iter_mut() {
        hunger.value += thread_rng().gen_range(10.0..80.0) * time.delta_seconds();
        println!("Eating! {:?}", &hunger.value);

        if hunger.value >= 100.0 {
            commands.entity(entity).remove::<Busy>();
        }
    }
}

fn begin_eat(mut commands: Commands, query: Query<(Entity), (Added<Enum!(AllTasks::Eat)>)>) {
    for entity in &query {
        commands.entity(entity).insert(Busy);
    }
}

fn check_wander(query: Query<(Entity, With<Character>, With<Enum!(AllTasks::Wander)>)>)
{
    println!("Wandering");
}

fn check_task(query: Query<(Entity, With<Character>, &AllTasks)>)
{
    //println!("TASKS:");
    //for (_, _, task) in &query {
    //println!("Task: {:?}", task);
    //}
}

impl Plugin for TaskScoringPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, score_basic_tasks)
            .add_systems(Update, eat.run_if(in_state(InGame)))
            .add_systems(Update, begin_eat.run_if(in_state(InGame)))
            .add_systems(Update, check_task);
    }
}