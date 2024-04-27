use bevy::app::{App, Plugin};
use bevy::asset::LoadedFolder;
use bevy::math::{uvec2, vec2, vec3};
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use bevy_fast_tilemap::*;
use rand::prelude::*;

use crate::{AppState, TerrainFolder};
use crate::growth_plugin::Growth;
use crate::pathing::Pos;

pub const SPRITE_SIZE: i32 = 32;
pub const WORLD_SIZE_X: i32 = 2048;
pub const WORLD_SIZE_Y: i32 = 2048;

#[allow(unused)]
#[derive(Component)]
pub struct Terrain;

#[derive(Component)]
struct AnimationLayer;

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
    //image_assets: Res<MyAssets>,
    terrain_sprites_handles: Res<TerrainFolder>,
    asset_server: Res<AssetServer>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut textures: ResMut<Assets<Image>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut materials: ResMut<Assets<Map>>,
) {
    let mut rand = thread_rng();

    // All the texture atlas stuff is from: https://github.com/bevyengine/bevy/blob/main/examples/2d/texture_atlas.rs
    let loaded_folder = loaded_folders.get(&terrain_sprites_handles.0).unwrap();

    let (texture_atlas_linear, linear_texture) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let atlas_linear_handle = texture_atlases.add(texture_atlas_linear.clone());

    let ugly_grass: Handle<Image> = asset_server.get_handle("terrain/ugly_grass.png").unwrap();
    let ugly_grass_index = texture_atlas_linear.get_texture_index(&ugly_grass).unwrap();

    let ugly_grass2: Handle<Image> = asset_server.get_handle("terrain/ugly_grass2.png").unwrap();
    let ugly_grass2_index = texture_atlas_linear
        .get_texture_index(&ugly_grass2)
        .unwrap();

    let ugly_grass3: Handle<Image> = asset_server.get_handle("terrain/ugly_grass3.png").unwrap();
    let ugly_grass3_index = texture_atlas_linear
        .get_texture_index(&ugly_grass3)
        .unwrap();

    let ugly_grass4: Handle<Image> = asset_server.get_handle("terrain/ugly_grass4.png").unwrap();
    let ugly_grass4_index = texture_atlas_linear
        .get_texture_index(&ugly_grass4)
        .unwrap();

    let ugly_mud: Handle<Image> = asset_server.get_handle("terrain/ugly_mud.png").unwrap();
    let ugly_mud_index = texture_atlas_linear.get_texture_index(&ugly_mud).unwrap();

    let ugly_mud2: Handle<Image> = asset_server.get_handle("terrain/ugly_mud2.png").unwrap();
    let ugly_mud2_index = texture_atlas_linear.get_texture_index(&ugly_mud2).unwrap();

    let ugly_mud3: Handle<Image> = asset_server.get_handle("terrain/ugly_mud3.png").unwrap();
    let ugly_mud3_index = texture_atlas_linear.get_texture_index(&ugly_mud3).unwrap();

    let ugly_mud4: Handle<Image> = asset_server.get_handle("terrain/ugly_mud4.png").unwrap();
    let ugly_mud4_index = texture_atlas_linear.get_texture_index(&ugly_mud4).unwrap();

    //let ugly_flower: Handle<Image> = asset_server.get_handle("terrain/ugly_flower.png").unwrap();
    /*let ugly_flower_index = texture_atlas_linear
    .get_texture_index(&ugly_flower)
    .unwrap();*/

    // includes grass duplicates to encourage grass growth
    let terrain_textures = vec![
        ugly_grass_index,
        ugly_grass2_index,
        ugly_grass3_index,
        ugly_grass4_index,
        ugly_grass_index,
        ugly_grass2_index,
        ugly_grass3_index,
        ugly_grass4_index,
        ugly_grass_index,
        ugly_grass2_index,
        ugly_grass3_index,
        ugly_grass4_index,
        ugly_grass_index,
        ugly_grass2_index,
        ugly_grass3_index,
        ugly_grass4_index,
        ugly_grass_index,
        ugly_grass2_index,
        ugly_grass3_index,
        ugly_grass4_index,
        ugly_mud_index,
        ugly_mud2_index,
        ugly_mud3_index,
        ugly_mud4_index,
    ];

    let map = Map::builder(
        uvec2(4096, 4096),
        linear_texture,
        vec2(SPRITE_SIZE as f32, SPRITE_SIZE as f32),
    )
        .build_and_initialize(|m| {
            // Initialize using a closure
            for y in 0..m.size().y {
                for x in 0..m.size().y {
                    m.set(x, y, rand.gen_range(0..2) + rand.gen_range(0..2) + rand.gen_range(0..1) + rand.gen_range(0..1) + rand.gen_range(0..1) as u32);
                }
            }
        });

    commands.spawn(MapBundleManaged {
        material: materials.add(map),
        ..default()
    });

    /*let map = Map::builder(
        uvec2(64, 64),
        linear_texture,
        vec2(16., 16.),
    )
        .build();*/

    /*let bundle = MapBundleManaged {
        material: materials.add(map),
        transform: Transform::default().with_translation(vec3(0., 0., 1.)),
        ..default()
    };*/

    //commands.spawn(bundle).insert(AnimationLayer);

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    //let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    //let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    /*let mut positions = vec![];
    let mut absolute_positions = vec![];
    let mut tile_positions = vec![];
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(
                        terrain_textures[rand.gen_range(0..terrain_textures.len())] as u32,
                    ),
                    ..default()
                }, ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);

            positions.push(Pos::new(x as i32, y as i32));
            absolute_positions.push((
                (x as i32 * SPRITE_SIZE) as i32,
                (y as i32 * SPRITE_SIZE) as i32,
            ));
            tile_positions.push(tile_pos);
        }
    }

    let tile_size = TilemapTileSize {
        x: SPRITE_SIZE as f32,
        y: SPRITE_SIZE as f32,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(linear_texture),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });*/

    next_state.set(AppState::InGame);
}

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Growth>()
            .add_systems(OnEnter(AppState::CreateWorld), create_world);
    }
}
