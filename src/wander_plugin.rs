use crate::character_plugin::Character;
use crate::tasks::*;
use crate::AppState;
use crate::AppState::Loading;
use bevy::app::App;
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
use bevy_enum_filter::prelude::*;
use rand::{thread_rng, Rng};
use crate::pathing::Pos;
use crate::world_gen_plugin::SPRITE_SIZE;

pub struct RandomMovementPlugin;

#[derive(Resource)]
struct ChangeDirTimer(Timer);

#[derive(Component)]
pub struct Wandering;

#[derive(Component)]
pub struct NeedsPath {
    pos: Pos
}

#[derive(Component)]
pub struct Path {
    path: (Vec::<Pos>, u32)
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
    for entity in &query {
        commands
            .entity(entity)
            .insert((Wandering, RandomDirection::default()));
    }
}

fn move_randomly(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &RandomDirection),
        (With<Wandering>, With<Enum!(AllTasks::Wander)>, Without<Path>),
    >,
) {
    let mut rand = thread_rng();
    for (entity, mut transform, dir) in query.iter_mut() {

        let start = Pos(transform.translation.x as i32, transform.translation.y as i32);
        let goal = Pos(rand.gen_range(0..255), rand.gen_range(0..255));

        let now = std::time::Instant::now();
        let path = pathfinding::prelude::astar(
            &start,
            |p| p.successors(),
            |p| p.distance(&goal) / 3,
            |p| *p == goal,
        );
        let elapsed_time = now.elapsed();
        println!("Getting a* path took {} ms.", elapsed_time.as_millis());
        commands.entity(entity).insert(Path {path: path.unwrap()});
    }
}

fn follow_path(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Path),
        (With<Wandering>, With<Enum!(AllTasks::Wander)>),
    >,
) {
    for (entity, mut transform, mut path) in query.iter_mut() {
        let mut next_pos = Vec3::new(path.path.0.iter().nth(0).unwrap().0 as f32 * 1 as f32, path.path.0.iter().nth(0).unwrap().1 as f32 * 1 as f32, transform.translation.z);

        if transform.translation.distance(next_pos) < 16.0 {
            //println!("Old target: {:?}", next_pos);
            // TODO: this is bad for performance since it shifts the full vec
            path.path.0.remove(0);

            if path.path.0.len() == 0 {
                println!("Done pathing");
                commands.entity(entity).remove::<Path>();
                continue;
            }
            next_pos = Vec3::new(path.path.0.iter().nth(0).unwrap().0 as f32 * 1 as f32, path.path.0.iter().nth(0).unwrap().1 as f32 * 1 as f32, transform.translation.z);
            //println!("New target: {:?}", next_pos);
        }

        let mut dir = next_pos - transform.translation;
        if let Some(d) = dir.try_normalize() {
           dir = d;
        }
        transform.translation.x += time.delta_seconds() * dir.x * 100.0;
        transform.translation.y += time.delta_seconds() * dir.y * 100.0;
    }
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
            //.add_systems(Update, update_random_dir.run_if(in_state(AppState::InGame)));
    }
}
