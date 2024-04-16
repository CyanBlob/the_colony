use bevy::app::{App, Plugin};
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;

use crate::{AppState, MyAssets, TerrainFolder};
use crate::growth_plugin::{Growth, Plant};

const SPRITE_SIZE: i32 = 32;
const WORLD_SIZE_X: i32 = 256;
const WORLD_SIZE_Y: i32 = 256;

#[derive(Component)]
pub struct Terrain;

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

/// Create and spawn a sprite from a texture atlas
fn create_sprite_from_atlas(
    commands: &mut Commands,
    translation: (f32, f32, f32),
    sprite_index: usize,
    atlas_handle: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(translation.0, translation.1, translation.2),
                scale: Vec3::splat(1.0),
                ..default()
            },
            texture,
            ..default()
        },
        TextureAtlas {
            layout: atlas_handle,
            index: sprite_index,
        },
    ));
}

fn create_world(
    mut commands: Commands,
    //image_assets: Res<MyAssets>,
    terrain_sprites_handles: Res<TerrainFolder>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
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
    let ugly_grass2_index = texture_atlas_linear.get_texture_index(&ugly_grass2).unwrap();

    let ugly_grass3: Handle<Image> = asset_server.get_handle("terrain/ugly_grass3.png").unwrap();
    let ugly_grass3_index = texture_atlas_linear.get_texture_index(&ugly_grass3).unwrap();

    let ugly_grass4: Handle<Image> = asset_server.get_handle("terrain/ugly_grass4.png").unwrap();
    let ugly_grass4_index = texture_atlas_linear.get_texture_index(&ugly_grass4).unwrap();

    let ugly_mud: Handle<Image> = asset_server.get_handle("terrain/ugly_mud.png").unwrap();
    let ugly_mud_index = texture_atlas_linear.get_texture_index(&ugly_mud).unwrap();

    let ugly_mud2: Handle<Image> = asset_server.get_handle("terrain/ugly_mud2.png").unwrap();
    let ugly_mud2_index = texture_atlas_linear.get_texture_index(&ugly_mud2).unwrap();

    let ugly_mud3: Handle<Image> = asset_server.get_handle("terrain/ugly_mud3.png").unwrap();
    let ugly_mud3_index = texture_atlas_linear.get_texture_index(&ugly_mud3).unwrap();

    let ugly_mud4: Handle<Image> = asset_server.get_handle("terrain/ugly_mud4.png").unwrap();
    let ugly_mud4_index = texture_atlas_linear.get_texture_index(&ugly_mud4).unwrap();

    let ugly_flower: Handle<Image> = asset_server.get_handle("terrain/ugly_flower.png").unwrap();
    let ugly_flower_index = texture_atlas_linear.get_texture_index(&ugly_flower).unwrap();

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


    let map_size = TilemapSize { x: WORLD_SIZE_X as u32, y: WORLD_SIZE_Y as u32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(terrain_textures[rand.gen_range(0..terrain_textures.len())] as u32),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: SPRITE_SIZE as f32, y: SPRITE_SIZE as f32 };
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
    });

    return;


    // world is centered at 0,0
    for x in WORLD_SIZE_X / -2..WORLD_SIZE_X / 2 {
        for y in WORLD_SIZE_Y / -2..WORLD_SIZE_Y / 2 {
            let i = rand.gen_range(0..terrain_textures.len());
            let texture = terrain_textures[i];

            create_sprite_from_atlas(
                &mut commands,
                ((x * SPRITE_SIZE) as f32, (y * SPRITE_SIZE) as f32, 0.0),
                texture,
                atlas_linear_handle.clone(),
                linear_texture.clone(),
            );

            /*commands.spawn((
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
                    inherited_visibility: Default::default(),
                    view_visibility: Default::default(),
                },
                Terrain,
            ));*/

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
                        texture: linear_texture.clone(),
                        visibility: Visibility::Visible,
                        inherited_visibility: Default::default(),
                        view_visibility: Default::default(),
                    },
                    TextureAtlas {
                        layout: atlas_linear_handle.clone(),
                        index: ugly_flower_index,
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
