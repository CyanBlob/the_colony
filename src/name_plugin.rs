use bevy::app::{App, Plugin};
use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Debug)]
pub struct Name(pub String);

#[derive(Component)]
pub struct NeedsName;

pub struct NamePlugin;

fn tick_pop(mut commands: Commands, mut query: Query<Entity, With<NeedsName>>) {
    let names = vec![
        "Alice", "Charlie", "Dave", "Eve", "Frank", "Grace", "Hank", "Iris", "Judy", "Karl",
        "Linda", "Mike", "Nancy", "Oscar", "Peggy", "Quinn", "Ruth", "Steve", "Tina", "Ursula",
        "Victor", "Wendy", "Xavier", "Yvonne", "Zach",
    ];

    for entity in query.iter_mut() {
        let i = rand::thread_rng().gen_range(0..names.len());
        let text_name = names[i];

        commands
            .entity(entity)
            .insert(Name(text_name.to_string()))
            .remove::<NeedsName>();
    }
}

impl Plugin for NamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_pop);
    }
}
