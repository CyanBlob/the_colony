use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_ecs_tilemap::prelude::TileStorage;

use crate::AppState::InGame;
use crate::character_plugin::Character;
use crate::world_gen_plugin::PathfindingRefs;

pub struct DebugPlugin;

fn debugCharacterPos(
    mut commands: Commands,
    query: Query<&Transform, With<Character>>,
    astarQuery: Query<&mut PathfindingRefs>,
    tileStorageQuery: Query<&TileStorage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for char in query.iter() {
        commands.spawn(MaterialMesh2dBundle {
            //mesh: Circle { radius: 10.0 },
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material: materials.add(Color::rgb(0., 1., 1.)),
            transform: Transform {
                translation: char.translation,
                rotation: Default::default(),
                scale: Vec3::splat(1.0),
            },
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        });
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debugCharacterPos.run_if(in_state(InGame)));
    }
}
