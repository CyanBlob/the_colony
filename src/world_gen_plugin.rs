use crate::growth_plugin::{Growth, Plant};
use crate::{AppState, MyAssets};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use rand::prelude::*;

const SPRITE_SIZE: i32 = 32;
const WORLD_SIZE_X: i32 = 128;
const WORLD_SIZE_Y: i32 = 128;

#[derive(Component)]
pub struct Terrain;

pub struct WorldGenPlugin;

fn create_world(mut commands: Commands, image_assets: Res<MyAssets>) {
    let mut rand = thread_rng();

    // includes grass duplicates to encourage grass growth
    let terrain_textures = vec![
        &image_assets.ugly_grass, &image_assets.ugly_grass2,
        &image_assets.ugly_grass3, &image_assets.ugly_grass4,
        &image_assets.ugly_grass, &image_assets.ugly_grass2,
        &image_assets.ugly_grass3, &image_assets.ugly_grass4,
        &image_assets.ugly_grass, &image_assets.ugly_grass2,
        &image_assets.ugly_grass3, &image_assets.ugly_grass4,
        &image_assets.ugly_grass, &image_assets.ugly_grass2,
        &image_assets.ugly_grass3, &image_assets.ugly_grass4,
        &image_assets.ugly_grass, &image_assets.ugly_grass2,
        &image_assets.ugly_grass3, &image_assets.ugly_grass4,
        &image_assets.ugly_grass, &image_assets.ugly_grass2,
        &image_assets.ugly_grass3, &image_assets.ugly_grass4,
        &image_assets.ugly_mud, &image_assets.ugly_mud2,
        &image_assets.ugly_mud3, &image_assets.ugly_mud4];

    // world is centered at 0,0
    for x in WORLD_SIZE_X / -2..WORLD_SIZE_X / 2 {
        for y in WORLD_SIZE_Y / -2..WORLD_SIZE_Y / 2 {
            let i = rand.gen_range(0..terrain_textures.len());
            let texture = terrain_textures[i];

            commands.spawn((
                SpriteBundle {
                    sprite: Default::default(),
                    transform: Transform::from_xyz(
                        (x * SPRITE_SIZE) as f32,
                        (y * SPRITE_SIZE) as f32,
                        0.0,
                    ),
                    global_transform: Default::default(),
                    texture: texture.clone(),
                    visibility: Visibility::Visible,
                    computed_visibility: Default::default(),
                },
                Terrain,
            ));

            if let true = rand.gen_bool(1.0 / 10.0) {
                // don't spawn plants on every tile
                commands.spawn((
                    SpriteBundle {
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
                    },
                    Plant,
                    Growth {
                        age: 0.0,
                        grow_rate: 1.0,
                    },
                ));
            }
        }
    }
}

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Growth>()
            .add_systems(OnEnter(AppState::InGame), create_world);
    }
}
