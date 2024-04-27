use std::sync::Mutex;

use bevy::app::App;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
//use bevy_ecs_tilemap::prelude::TileStorage;
use bevy_enum_filter::prelude::*;
use rand::{Rng, thread_rng};

use crate::AppState;
use crate::AppState::Loading;
use crate::character_plugin::Character;
use crate::pathing::Pos;
use crate::tasks::*;
use crate::world_gen_plugin::{SPRITE_SIZE, TileWeights, WORLD_SIZE_X, WORLD_SIZE_Y};

pub struct RandomMovementPlugin;

#[derive(Resource)]
struct ChangeDirTimer(Timer);

#[derive(Component)]
pub struct Wandering;

#[derive(Component)]
pub struct NeedsPath {
    pos: Pos,
}

#[derive(Component)]
pub struct Path {
    path: (Vec::<Pos>, u32),
}

#[derive(Component, Default)]
pub struct RandomDirection {
    dir: Vec2,
}

fn wander(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<Character>,
            With<Enum!(AllTasks::Wander)>,
            Without<Wandering>,
        ),
    >,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((Wandering, RandomDirection::default()));
    }
}

fn move_randomly(
    mut commands: Commands,
    query: Query<
        (Entity, &Transform),
        (With<Wandering>, With<Enum!(AllTasks::Wander)>, Without<Path>),
    >,
    weights_query: Query<&TileWeights>,
) {
    let paths: Mutex<Vec<(Entity, Path)>> = Mutex::new(vec![]);

    let weights = &weights_query.get_single().unwrap().weights;
    let weights = Mutex::new(weights);

    query.par_iter().for_each(|(entity, transform)| {
        let mut rand = thread_rng();

        let start = Pos(transform.translation.x as i32 / 32, transform.translation.y as i32 / 32);
        let goal = Pos(rand.gen_range(-127..127), rand.gen_range(-127..127));

        let path = pathfinding::prelude::astar(
            &start,
            |p| p.successors(weights.lock().unwrap()),
            |p| p.distance(&goal) / 3,
            |p| *p == goal,
        ).unwrap();
        paths.lock().unwrap().push((entity, Path { path: path }));
    });

    for (entity, path) in paths.lock().unwrap().iter() {
        commands.entity(*entity).insert(Path { path: path.path.clone() });
    }
}

fn follow_path(
    time: Res<Time>,
    commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Path),
        (With<Wandering>, With<Enum!(AllTasks::Wander)>),
    >,
    //tile_storage_query: Query<&TileStorage>,
) {
    let commands = Mutex::new(commands);
    //let tile_storage = tile_storage_query.get_single().unwrap();

    query.par_iter_mut().for_each(|(entity, mut transform, mut path)| {
        let mut next_pos = Vec3::new(
            path.path.0.iter().nth(0).unwrap().0 as f32 * SPRITE_SIZE as f32 - (0) as f32 / 2.0,
            path.path.0.iter().nth(0).unwrap().1 as f32 * SPRITE_SIZE as f32 - (0) as f32 / 2.0,
            transform.translation.z,
        );


        if transform.translation.distance(next_pos) < 32.0 {
            //println!("Old target: {:?}", next_pos);
            // TODO: this is bad for performance since it shifts the full vec
            path.path.0.remove(0);

            if path.path.0.len() == 0 {
                commands.lock().unwrap().entity(entity).remove::<Path>();
                return;
            }
            next_pos = Vec3::new(
                path.path.0.iter().nth(0).unwrap().0 as f32 * SPRITE_SIZE as f32 - (WORLD_SIZE_X as u32 * SPRITE_SIZE as u32) as f32 / 2.0,
                path.path.0.iter().nth(0).unwrap().1 as f32 * SPRITE_SIZE as f32 - (WORLD_SIZE_Y as u32 * SPRITE_SIZE as u32) as f32 / 2.0,
                transform.translation.z,
            );
        }

        let mut dir = next_pos - transform.translation;
        if let Some(d) = dir.try_normalize() {
            dir = d;
        }
        transform.translation.x += time.delta_seconds() * dir.x * 200.0;
        transform.translation.y += time.delta_seconds() * dir.y * 200.0;
    });
}

fn update_random_dir(
    time: Res<Time>,
    mut timer: ResMut<ChangeDirTimer>,
    mut query: Query<&mut RandomDirection, (With<Wandering>, With<Enum!(AllTasks::Wander)>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for dir in query.iter_mut() {
            update_random_dir_base(dir);
        }
    }
}

fn update_random_dir_without_tick(mut query: Query<&mut RandomDirection>) {
    for dir in query.iter_mut() {
        update_random_dir_base(dir);
    }
}

fn update_random_dir_base(mut dir: Mut<RandomDirection>) {
    dir.dir.x = thread_rng().gen_range(-10.0..10.0);
    dir.dir.y = thread_rng().gen_range(-10.0..10.0);
}

impl Plugin for RandomMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChangeDirTimer(Timer::from_seconds(2.0, Repeating)))
            .add_systems(OnExit(Loading), update_random_dir_without_tick)
            .add_systems(Update, wander.run_if(in_state(AppState::InGame)))
            .add_systems(Update, move_randomly.run_if(in_state(AppState::InGame)))
            .add_systems(Update, follow_path.run_if(in_state(AppState::InGame)));
    }
}
