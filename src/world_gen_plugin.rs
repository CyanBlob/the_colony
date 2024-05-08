use bevy::app::{App, Plugin};
use bevy::asset::LoadedFolder;
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use bevy::utils::HashMap;
use bevy_fast_tilemap::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::{AppState, TerrainFolder};
use crate::growth_plugin::Growth;
use crate::pathing::Pos;

pub const SPRITE_SIZE: i32 = 32;
pub const WORLD_SIZE_X: i32 = 512;
pub const WORLD_SIZE_Y: i32 = 512;

#[allow(unused)]
#[derive(Component)]
pub struct Terrain;

#[allow(unused)]
#[derive(Component)]
struct AnimationLayer;

#[derive(Resource)]
pub struct TileWeights {
    pub weights: HashMap::<Pos, i32>,
}

pub struct WorldGenPlugin;

pub(crate) fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture)
}

fn create_world(
    mut commands: Commands,
    terrain_sprites_handles: Res<TerrainFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut materials: ResMut<Assets<Map>>,
) {
    let mut rand = thread_rng();

    // All the texture atlas stuff is from: https://github.com/bevyengine/bevy/blob/main/examples/2d/texture_atlas.rs
    let loaded_folder = loaded_folders.get(&terrain_sprites_handles.0).unwrap();

    let (_, linear_texture) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()),
        &mut textures,
    );

    const NUM_TERRAIN_TILES: usize = 7;
    let mut weights = [1; NUM_TERRAIN_TILES];

    for i in 0..NUM_TERRAIN_TILES {
        weights[i] = NUM_TERRAIN_TILES - i;
    }
    let dist = WeightedIndex::new(&weights).unwrap();

    let mut hashmap = HashMap::new();

    let map = Map::builder(
        uvec2(WORLD_SIZE_X as u32, WORLD_SIZE_Y as u32),
        linear_texture,
        vec2(SPRITE_SIZE as f32, SPRITE_SIZE as f32),
    )
        .build_and_initialize(|m| {
            // Initialize using a closure
            for y in 0..m.size().y {
                for x in 0..m.size().y {
                    m.set(x, y, dist.sample(&mut rand) as u32);
                    //hashmap.insert(Pos(x as i32, y as i32), rand.gen_range(1..255));
                    hashmap.insert(Pos(x as i32, y as i32), 1);
                }
            }
        });

    commands.spawn(MapBundleManaged {
        material: materials.add(map),
        ..default()
    });

    commands.insert_resource(TileWeights { weights: hashmap });

    next_state.set(AppState::InGame);
}

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Growth>()
            .add_systems(OnEnter(AppState::CreateWorld), create_world);
    }
}
