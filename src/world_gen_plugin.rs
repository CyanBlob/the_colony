use crate::{AppState, MyAssets};
use bevy::app::{App, Plugin};
use bevy::prelude::*;

const SPRITE_SIZE: i32 = 32;
const WORLD_SIZE_X: i32 = 250;
const WORLD_SIZE_Y: i32 = 250;

#[derive(Component)]
pub struct Terrain;

pub struct WorldGenPlugin;

fn create_world(mut commands: Commands, image_assets: Res<MyAssets>) {
    println!("CREATING WORLD");
    // world is centered at 0,0
    for x in WORLD_SIZE_X / -2..WORLD_SIZE_X / 2 {
        for y in WORLD_SIZE_Y / -2..WORLD_SIZE_Y / 2 {
            commands.spawn((
                SpriteBundle {
                    sprite: Default::default(),
                    transform: Transform::from_xyz(
                        (x * SPRITE_SIZE) as f32,
                        (y * SPRITE_SIZE) as f32,
                        0.0,
                    ),
                    global_transform: Default::default(),
                    texture: image_assets.ugly_grass.clone(),
                    visibility: Visibility::Visible,
                    computed_visibility: Default::default(),
                },
                Terrain,
            ));
            commands.spawn(SpriteBundle {
                sprite: Default::default(),
                transform: Transform::from_xyz(
                    (x * SPRITE_SIZE) as f32,
                    (y * SPRITE_SIZE) as f32,
                    1.0,
                ),
                global_transform: Default::default(),
                texture: image_assets.ugly_flower.clone(),
                visibility: Visibility::Visible,
                computed_visibility: Default::default(),
            });
        }
    }
}

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), create_world);
    }
}
