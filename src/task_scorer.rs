use crate::character_plugin::Character;
use crate::name_plugin::Name;
use crate::tasks::*;
use crate::AppState::InGame;
use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_enum_filter::Enum;

pub trait Task {
    fn score(&self) -> f32;
}

pub struct TaskScoringPlugin;

#[derive(Component)]
pub struct Busy;

fn score_basic_tasks(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Name, &mut AllTasks, &Thirst, &Hunger, &Sleep), Without<Busy>>,
) {
    for (entity, name, mut task, thirst, hunger, sleep) in query.iter_mut() {
        let mut ratings = vec![(AllTasks::Wander, 1.0)];

        ratings.push((AllTasks::Eat, hunger.score()));
        ratings.push((AllTasks::Drink, thirst.score()));
        ratings.push((AllTasks::Sleep, sleep.score()));

        ratings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        *task = ratings.iter().nth(0).unwrap().0;

        // update "Busy" flag. Adding this causes the entity to be skipped in the scoring query
        match *task {
            AllTasks::Wander => {
                commands.entity(entity).remove::<Busy>();
            }
            _ => {
                screen_print!(push, sec: 3.0, "{}: {:?}", &name.0, *task);
                commands.entity(entity).insert(Busy);
            }
        }
    }
}

fn begin_eat(mut commands: Commands, query: Query<Entity, Added<Enum!(AllTasks::Eat)>>) {
    for entity in &query {
        commands.entity(entity).insert(Busy);
    }
}

fn check_wander(query: Query<Entity, (With<Character>, With<Enum!(AllTasks::Wander)>)>) {
    println!("Wandering");
}

fn check_task(query: Query<(Entity, &AllTasks), With<Character>>) {
    //println!("TASKS:");
    //for (_, _, task) in &query {
    //println!("Task: {:?}", task);
    //}
}

fn render_task_text(
    p_query: Query<(Entity, &Children, &AllTasks, &Name), With<Character>>,
    mut c_query: Query<&mut Text>,
) {
    for (_, children, task, name) in p_query.iter() {
        // `children` is a collection of Entity IDs
        for &child in children.iter() {
            // get the text child
            let text = c_query.get_mut(child);

            match text {
                Ok(mut t) => {
                    t.sections.clear();
                    t.sections.push(TextSection {
                        value: format!("{}\n{}", &name.0, task.as_ref().to_string()),
                        style: Default::default(),
                    });
                }
                Err(_) => {}
            }
        }
    }
}

impl Plugin for TaskScoringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, score_basic_tasks)
            //.add_systems(Update, begin_eat.run_if(in_state(InGame)))
            .add_systems(Update, render_task_text.run_if(in_state(InGame)))
            .add_systems(Update, check_task);
    }
}
