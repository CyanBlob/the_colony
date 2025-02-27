use std::sync::Mutex;

use bevy::app::App;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task};
use bevy::tasks::futures_lite::{future};
use bevy_enum_filter::prelude::*;
use rand::{Rng, thread_rng};

use crate::AppState;
use crate::character_plugin::Character;
use crate::pathing::Pos;
use crate::tasks::*;
use crate::world_gen_plugin::{SPRITE_SIZE, TileWeights, WORLD_SIZE_X, WORLD_SIZE_Y};

pub struct RandomMovementPlugin;

#[derive(Component)]
pub struct Wandering;

#[derive(Component)]
pub struct PathPending;

#[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
pub struct NeedsPath {
    pub pos: Pos,
}

#[derive(Component)]
pub struct Path {
    path: (Vec::<Pos>, u32),
    index: usize,
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
            .insert(Wandering);
    }
}

fn assign_path(
    mut commands: Commands,
    query: Query<
        (Entity, &Transform, &NeedsPath),
        (With<Transform>, With<Wandering>, With<Enum!(AllTasks::Wander)>, Without<Path>, Without<PathPending>),
    >,
    weights: Res<TileWeights>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    let entities: Vec::<(Entity, Transform, NeedsPath)> = query.iter().map(|(entity, transform, needs_path)| { (entity, transform.clone(), needs_path.clone()) }).collect();

    for (entity, transform, needs_path) in entities.iter() {
        let entity = entity.clone();
        commands.entity(entity).insert(PathPending);
        commands.entity(entity).remove::<NeedsPath>();

        let weights = weights.weights.clone();
        let transform = transform.clone();
        let needs_path = needs_path.clone();

        let task = thread_pool.spawn(async move {
            let mut command_queue = CommandQueue::default();

            let start = Pos(transform.translation.x as i32 / 32, transform.translation.y as i32 / 32);

            let path = pathfinding::prelude::astar(
                &start,
                |p| p.successors(&weights),
                |p| p.distance(&needs_path.pos) / 1,
                |p| *p == needs_path.pos,
            ).unwrap();

            command_queue.push(move |world: &mut World| {
                world.entity_mut(entity).insert(Path { path: path.clone(), index: 0 }).remove::<ComputeTransform>().remove::<PathPending>();
            });
            command_queue
        });

        commands.entity(entity).insert(ComputeTransform(task));
    }
}

// polls the async pathing tasks until they finish, then applies the commands
fn handle_tasks(mut commands: Commands, mut transform_tasks: Query<&mut ComputeTransform>) {
    for mut task in &mut transform_tasks {
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
                commands.lock().unwrap().entity(entity).remove::<Path>();
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

fn set_wander_goal(
    mut commands: Commands,
    query: Query<
        Entity,
        (With<Transform>, With<Wandering>, With<Enum!(AllTasks::Wander)>, Without<Path>, Without<NeedsPath>, Without<PathPending>),
    >,
) {
    for entity in query.iter() {
        let mut rng = thread_rng();
        let goal = Pos(rng.gen_range(-127..127), rng.gen_range(-127..127));
        commands.entity(entity).insert(NeedsPath { pos: goal });
    }
}

impl Plugin for RandomMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wander.run_if(in_state(AppState::InGame)))
            .add_systems(Update, assign_path.run_if(in_state(AppState::InGame)))
            .add_systems(Update, follow_path.run_if(in_state(AppState::InGame)))
            .add_systems(Update, set_wander_goal.run_if(in_state(AppState::InGame)))
            .add_systems(Update, handle_tasks);
    }
}
