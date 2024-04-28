use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::AppState::InGame;
use crate::character_plugin::Character;

//use bevy_ecs_tilemap::prelude::TileStorage;

pub struct DebugPlugin;

#[derive(Component)]
pub struct DrawCircleOneFrame {
    pub(crate) pos: Vec2,
    pub radius: f32,
    pub(crate) color: Color,
}

#[derive(Component)]
pub struct DebugDespawn;

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

fn draw_circle_one_frame(mut commands: Commands,
                         mut materials: ResMut<Assets<ColorMaterial>>,
                         mut meshes: ResMut<Assets<Mesh>>,
                         query: Query<(Entity, &DrawCircleOneFrame)>) {
    for (entity, draw) in query.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: draw.radius })),
            material: materials.add(draw.color),
            transform: Transform {
                translation: Vec3::new(
                    draw.pos.x,
                    draw.pos.y,
                    500.0,
                ),
                rotation: Default::default(),
                scale: Vec3::splat(1.0),
            },
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        }).insert(DebugDespawn);
        commands.entity(entity).remove::<DrawCircleOneFrame>().insert(DebugDespawn);
    }
}

fn debug_despawn(
    mut commands: Commands,
    query: Query<Entity, With<DebugDespawn>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/*fn visualize_astar(
    mut commands: Commands,
    tile_storage_query: Query<&TileStorage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let now = std::time::Instant::now();
    let start = Pos(0, 0);
    let goal = Pos(255, 255);

    let path = pathfinding::prelude::astar(
        &start,
        |p| p.successors(),
        |p| p.distance(&goal) / 3,
        |p| *p == goal,
    );
    let elapsed_time = now.elapsed();
    println!("Getting a* path took {} ms.", elapsed_time.as_millis());

    let tile_storage = tile_storage_query.get_single().unwrap();

    for pos in path.unwrap().0.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material: materials.add(Color::rgb(0., 1., 1.)),
            transform: Transform {
                translation: Vec3::new(
                    pos.0 as f32 * SPRITE_SIZE as f32
                        - (tile_storage.size.x * SPRITE_SIZE as u32) as f32 / 2.,
                    pos.1 as f32 * SPRITE_SIZE as f32
                        - (tile_storage.size.y * SPRITE_SIZE as u32) as f32 / 2.,
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
}*/

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_despawn.run_if(in_state(InGame)));
        app.add_systems(Update, draw_circle_one_frame.run_if(in_state(InGame)));
        //app.add_systems(Update, debug_character_pos.run_if(in_state(InGame)));
        //app.add_systems(OnEnter(AppState::InGame), visualize_astar);
    }
}
