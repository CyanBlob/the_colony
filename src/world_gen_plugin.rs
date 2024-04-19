use std::sync::{Arc, Mutex};

use bevy::app::{App, Plugin};
use bevy::asset::LoadedFolder;
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy::prelude::Keyframes::Translation;
use bevy::render::mesh::CircleMeshBuilder;
use bevy::render::texture::ImageSampler;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::petgraph::{Graph, Undirected};
use bevy::utils::petgraph::algo::astar;
use bevy::utils::petgraph::prelude::NodeIndex;
use bevy_ecs_tilemap::helpers::square_grid::*;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
use bevy_ecs_tilemap::prelude::*;
use bevy_enum_filter::Enum;
use rand::prelude::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{AppState, CharacterFolder, MyAssets, TerrainFolder};
use crate::AppState::InGame;
use crate::character_plugin::Character;
use crate::growth_plugin::{Growth, Plant};
use crate::task_scorer::Busy;
use crate::tasks::{Hunger, Thirst};

const SPRITE_SIZE: i32 = 32;
const WORLD_SIZE_X: i32 = 256;
const WORLD_SIZE_Y: i32 = 256;

#[derive(Component)]
pub struct Terrain;

#[derive(Component)]
pub struct AstarId {
    id: NodeIndex,
}

#[derive(Component)]
pub struct PathfindingRefs {
    aStar: Graph<i32, i32, Undirected>,
}

#[derive(Component)]
pub struct NeedsAstarNeighbors;

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
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut rand = thread_rng();

    let mut aStar = Graph::<i32, i32, Undirected>::new_undirected();
    /*let a = aStar.add_node(0);
    let b = aStar.add_node(0);
    let c = aStar.add_node(0);
    let d = aStar.add_node(0);
    let e = aStar.add_node(0);
    let f = aStar.add_node(0);
    aStar.extend_with_edges(&[
        (a, b, 2),
        (a, d, 4),
        (b, c, 1),
        (b, f, 7),
        (c, e, 5),
        (e, f, 1),
        (d, e, 1),
    ]);*/

    //assert_eq!(path, Some((6, vec![a, d, e, f])));

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

    let ugly_flower: Handle<Image> = asset_server.get_handle("terrain/ugly_flower.png").unwrap();
    let ugly_flower_index = texture_atlas_linear
        .get_texture_index(&ugly_flower)
        .unwrap();

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

    let map_size = TilemapSize {
        x: WORLD_SIZE_X as u32,
        y: WORLD_SIZE_Y as u32,
    };

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
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(
                            terrain_textures[rand.gen_range(0..terrain_textures.len())] as u32,
                        ),
                        ..default()
                    },
                    AstarId {
                        id: aStar.add_node(1),
                    },
                    NeedsAstarNeighbors,
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
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
    });

    commands.spawn(PathfindingRefs { aStar });

    next_state.set(AppState::InGame);
}

fn addAstarNeighbors(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TilePos, &mut AstarId), With<NeedsAstarNeighbors>>,
    mut astarQuery: Query<&mut PathfindingRefs>,
    mut tileStorageQuery: Query<&TileStorage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if query.iter().len() == 0 {
        return;
    }
    let all_tiles = &query.iter().collect::<Vec<_>>();

    let mut aStar = &mut astarQuery.get_single_mut().unwrap().aStar;

    //let mut aStar_mutex = Mutex::new(Arc::new(&astarQuery.get_single_mut().unwrap().aStar));
    //let mut aStar_mutex = Mutex::new((&astarQuery.get_single_mut().unwrap().aStar));
    //let mut aStar_mutex = Arc::new(Mutex::new(astarQuery.get_single_mut().unwrap().aStar));

    let edges_mutex = Mutex::new(vec![]);

    let map_size = TilemapSize {
        x: WORLD_SIZE_X as u32,
        y: WORLD_SIZE_Y as u32,
    };

    let tile_storage = tileStorageQuery.get_single().unwrap();

    query.par_iter().for_each(|(entity, tile_pos, astar_id)| {
        for neighbor in
        Neighbors::get_square_neighboring_positions(&tile_pos, &map_size, true).iter()
        {
            //let neighbor_entity = all_tiles.iter().find(|(a, b, c)| b == &neighbor);

            let neighbor_entity = tile_storage.get(&neighbor);

            let neighbor_entity = query.get(neighbor_entity.unwrap()).unwrap();

            //let mut aStar = aStar_mutex.lock().unwrap();

            //aStar.add_edge(astar_id.id, neighbor_entity.2.id, 1);
            edges_mutex.lock().unwrap().push((astar_id.id, neighbor_entity.2.id, 1, entity));
        }
        //commands.entity(entity).remove::<NeedsAstarNeighbors>();
    });

    for edge in edges_mutex.lock().unwrap().iter() {
        aStar.add_edge(edge.0, edge.1, edge.2);
        commands.entity(edge.3).remove::<NeedsAstarNeighbors>();
    }


    // TODO: This can probably be sped up
    //for (entity, tile_pos, astar_id) in query.iter() {

    //}
    let aStar = &astarQuery.get_single_mut().unwrap().aStar;

    let path = astar(
        &aStar,
        query.iter().nth(0).unwrap().2.id,
        |finish| finish == query.iter().nth(query.iter().len() - 1).unwrap().2.id,
        |e| *e.weight(),
        |_| 0,
    );

    for step in path.unwrap().1.iter() {
        let pos = all_tiles.par_iter().find_any(|(a, b, c)| &c.id == step);

        commands.spawn(MaterialMesh2dBundle {
            //mesh: Circle { radius: 10.0 },
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material: materials.add(Color::rgb(0., 1., 1.)),
            transform: Transform {
                translation: Vec3::new(
                    pos.unwrap().1.x as f32 * SPRITE_SIZE as f32 - (tile_storage.size.x * SPRITE_SIZE as u32) as f32 / 2.,
                    pos.unwrap().1.y as f32 * SPRITE_SIZE as f32 - (tile_storage.size.y * SPRITE_SIZE as u32) as f32 / 2.,
                    100.0,
                ),
                rotation: Default::default(),
                scale: Vec3::splat(1.0),
            },
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        });
    }
}

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Growth>()
            .add_systems(OnEnter(AppState::CreateWorld), create_world)
            .add_systems(Update, addAstarNeighbors.run_if(in_state(InGame)));
    }
}
