use std::future::pending;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use bevy::app::App;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task};
use bevy::tasks::futures_lite::{future, FutureExt};
use bevy::tasks::futures_lite::future::ready;
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
    pub pos: Pos,
}

#[derive(Component)]
pub struct Path {
    path: (Vec::<Pos>, u32),
    index: usize,
}

#[derive(Component, Default)]
pub struct RandomDirection {
    dir: Vec2,
}

#[derive(Component)]
struct ComputeTransform(Task<CommandQueue>);

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
        (With<Transform>, With<Wandering>, With<Enum!(AllTasks::Wander)>, Without<Path>, With<NeedsPath>),
    >,
    weights: Res<TileWeights>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    let entities: Vec::<(Entity, Transform)> = query.iter().map(|(entity, transform)| { (entity, transform.clone()) }).collect();

    for (entity, transform) in entities.iter() {
        let entity = entity.clone();
        commands.entity(entity).remove::<NeedsPath>();
        
        let weights = weights.weights.clone();
        let transform = transform.clone();

        let task = thread_pool.spawn(async move {
            let mut command_queue = CommandQueue::default();

            let mut rand = thread_rng();

            let start = Pos(transform.translation.x as i32 / 32, transform.translation.y as i32 / 32);
            let goal = Pos(rand.gen_range(-127..127), rand.gen_range(-127..127));

            let path = pathfinding::prelude::astar(
                &start,
                |p| p.successors(&weights),
                |p| p.distance(&goal) / 1,
                |p| *p == goal,
            ).unwrap();

            command_queue.push(move |world: &mut World| {
                world.entity_mut(entity).insert(Path { path: path.clone(), index: 0 }).remove::<ComputeTransform>();
            });
            command_queue
        });

        commands.entity(entity).insert(ComputeTransform(task));
    }
}

fn handle_tasks(mut commands: Commands, mut transform_tasks: Query<(&mut ComputeTransform)>) {
    for (mut task) in &mut transform_tasks {
        if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
    }
}

fn follow_path(
    time: Res<Time>,
    commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Path),
        (With<Wandering>, With<Enum!(AllTasks::Wander)>),
    >,
) {
    let commands = Mutex::new(commands);

    query.par_iter_mut().for_each(|(entity, mut transform, mut path)| {
        let mut next_pos = Vec3::new(
            path.path.0.iter().nth(path.index).unwrap().0 as f32 * SPRITE_SIZE as f32 - (0) as f32 / 2.0,
            path.path.0.iter().nth(path.index).unwrap().1 as f32 * SPRITE_SIZE as f32 - (0) as f32 / 2.0,
            transform.translation.z,
        );


        if transform.translation.distance(next_pos) < 32.0 {
            path.index += 1;

            if path.path.0.len() == path.index {
                commands.lock().unwrap().entity(entity).remove::<Path>().insert(NeedsPath { pos: Pos(0, 0) });
                return;
            }
            next_pos = Vec3::new(
                path.path.0.iter().nth(path.index).unwrap().0 as f32 * SPRITE_SIZE as f32 - (WORLD_SIZE_X as u32 * SPRITE_SIZE as u32) as f32 / 2.0,
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
            .add_systems(Update, follow_path.run_if(in_state(AppState::InGame)))
            .add_systems(Update, handle_tasks);
    }
}
