use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::ActionState;

use crate::AppState::InGame;
use crate::world_gen_plugin::SPRITE_SIZE;

pub struct InputPlugin;

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Spawn,
    Despawn,
}

#[derive(Component)]
struct GlobalInput;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<MyWorldCoords>()
            .add_systems(Startup, setup)
            .add_systems(Update, my_cursor_system)
            .add_systems(Update, jump.run_if(in_state(InGame)));
    }
}

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

fn setup(mut commands: Commands) {
    // Describes how to convert from player inputs into those actions
    let input_map = InputMap::new([(Action::Spawn, MouseButton::Left), (Action::Despawn, MouseButton::Right)]);
    commands
        .spawn(InputManagerBundle::with_map(input_map))
        .insert(GlobalInput);
}

// Query for the `ActionState` component in your game logic systems!
fn jump(query: Query<&ActionState<Action>, With<GlobalInput>>,
        asset_server: Res<AssetServer>,
        cursor_pos: Res<MyWorldCoords>,
        mut commands: Commands,
) {
    let action_state = query.single();

    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(&Action::Spawn) {
        let ugly_flower: Handle<Image> = asset_server.get_handle("plants/ugly_flower.png").unwrap();

        let sprite_size = SPRITE_SIZE as f32;

        let x_offset = match cursor_pos.0.x {
            x if x < 0.0 => { sprite_size / -2.0 }
            x if x >= 0.0 => { sprite_size / 2.0 }
            _ => { 0.0 }
        };

        let y_offset = match cursor_pos.0.y {
            y if y < 0.0 => { sprite_size / -2.0 }
            y if y >= 0.0 => { sprite_size / 2.0 }
            _ => { 0.0 }
        };

        let cursor_x = (cursor_pos.0.x as i32 / SPRITE_SIZE) as f32 * sprite_size + x_offset;
        let cursor_y = (cursor_pos.0.y as i32 / SPRITE_SIZE) as f32 * sprite_size + y_offset;

        commands.spawn(SpriteBundle {
            sprite: Default::default(),
            transform: Transform {
                translation: Vec3::new(
                    cursor_x,
                    cursor_y,
                    500.0,
                ),
                rotation: Default::default(),
                scale: Vec3::splat(1.0),
            },
            global_transform: Default::default(),
            texture: ugly_flower,
            visibility: Visibility::Visible,
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        });
    }
    if action_state.just_pressed(&Action::Despawn) {}
}
