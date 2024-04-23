use crate::world_gen_plugin::SPRITE_SIZE;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_ecs_tilemap::prelude::TileStorage;
use crate::AppState;

use crate::character_plugin::Character;
use crate::pathing::Pos;

pub struct DebugPlugin;

#[allow(unused)]
fn debug_character_pos(
    mut commands: Commands,
    query: Query<&Transform, With<Character>>,
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
fn visualize_astar(
    mut commands: Commands,
    tile_storage_query: Query<&TileStorage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let now = std::time::Instant::now();
    let start = Pos(0, 0);
    let goal = Pos(255, 255);

    let path = pathfinding::prelude::astar(&start, |p| p.successors(), |p| p.distance(&goal) / 3,
                                           |p| *p == goal);
    let elapsed_time = now.elapsed();
    println!("Getting a* path took {} ms.", elapsed_time.as_millis());

    let tile_storage = tile_storage_query.get_single().unwrap();

    for pos in path.unwrap().0.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material: materials.add(Color::rgb(0., 1., 1.)),
            transform: Transform {
                translation: Vec3::new(
                    pos.0 as f32 * SPRITE_SIZE as f32 - (tile_storage.size.x * SPRITE_SIZE as u32) as f32 / 2.,
                    pos.1 as f32 * SPRITE_SIZE as f32 - (tile_storage.size.y * SPRITE_SIZE as u32) as f32 / 2.,
                    100.0,
                ),
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
       //app.add_systems(Update, debug_character_pos.run_if(in_state(InGame)));
        app.add_systems(OnEnter(AppState::InGame), visualize_astar);
    }
}
