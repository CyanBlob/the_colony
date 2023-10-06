use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component)]
pub struct Plant;

#[derive(Reflect, Resource, Default, InspectorOptions, Component)]
#[reflect(Resource, InspectorOptions)]
pub struct Growth {
    pub age: f32,
    pub grow_rate: f32,
}

pub struct PlanGrowth;

fn grow_tick(time: Res<Time>, mut query: Query<&mut Growth, With<Plant>>) {
    for mut growable in query.iter_mut() {
        growable.age += growable.grow_rate * time.delta_seconds();
    }
}

impl Plugin for PlanGrowth {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grow_tick);
    }
}
