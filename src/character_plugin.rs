use bevy::app::{App, Plugin};
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use rand::{Rng, thread_rng};

use crate::{AppState, CharacterFolder};
use crate::name_plugin::NeedsName;
use crate::pathing::Pos;
use crate::tasks::*;
use crate::wander_plugin::NeedsPath;

#[derive(Component)]
pub struct Character;

#[derive(Bundle)]
struct PlayerBundle {
    character: Character,
    sprite: SpriteBundle,
    thirst: Thirst,
    hunger: Hunger,
    sleep: Sleep,
    target_task: AllTasks,
}

pub struct CharacterPlugin;

fn add_people(
    mut commands: Commands,
    //image_assets: Res<MyAssets>,
    character_sprite_handles: Res<CharacterFolder>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut rand = thread_rng();
    let loaded_folder = loaded_folders.get(&character_sprite_handles.0).unwrap();

    let (texture_atlas_linear, linear_texture) = crate::world_gen_plugin::create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let atlas_linear_handle = texture_atlases.add(texture_atlas_linear.clone());

    let character: Handle<Image> = asset_server.get_handle("characters/character.png").unwrap();
    let character_index = texture_atlas_linear.get_texture_index(&character).unwrap();

    for _ in 0..10 {
        let transform = Transform::from_xyz(
            rand.gen_range(-1000.0..1000.0),
            rand.gen_range(-640.0..640.0),
            100.0,
        );

        //println!("Spawning at: {:?}", transform);
        commands
            .spawn((
                PlayerBundle {
                    sprite: SpriteBundle {
                        texture: linear_texture.clone(),
                        transform: transform,
                        ..default()
                    },
                    character: Character,
                    thirst: Thirst::default(),
                    hunger: Hunger::default(),
                    sleep: Sleep::default(),
                    target_task: AllTasks::default(),
                },
                TextureAtlas {
                    layout: atlas_linear_handle.clone(),
                    index: character_index,
                },
                NeedsName,
                NeedsPath {
                   pos: Pos(0, 0) 
                }
                /*Text2dBundle {
                    transform: Default::default(),
                    text_anchor: Default::default(),
                    text_2d_bounds: Default::default(),
                    global_transform: Default::default(),
                    visibility: Default::default(),
                    computed_visibility: Default::default(),
                    text_layout_info: Default::default(),
                }*/
            ))
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section("TEST TEXT", TextStyle::default()),
                    text_anchor: Default::default(),
                    text_2d_bounds: Default::default(),
                    transform: Transform::from_xyz(0.0, 20.0, 10.0),
                    global_transform: Default::default(),
                    visibility: Default::default(),
                    inherited_visibility: Default::default(),
                    view_visibility: Default::default(),
                    text_layout_info: Default::default(),
                });
            });
    }

    return;

    /*for _ in 0..10 {
        commands.spawn((
            PlayerBundle {
                sprite: SpriteBundle {
                    texture: assets.character.clone(),
                    transform: Transform::from_xyz(
                        rand.gen_range(-1000.0..1000.0),
                        rand.gen_range(-640.0..640.0),
                        100.0,
                    ),
                    ..default()
                },
                character: Character,
                thirst: Thirst::default(),
                hunger: Hunger::default(),
                sleep: Sleep::default(),
                target_task: AllTasks::default(),
            },
            NeedsName,
            /*Text2dBundle {
                transform: Default::default(),
                text_anchor: Default::default(),
                text_2d_bounds: Default::default(),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
                text_layout_info: Default::default(),
            }*/
        )).with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section("TEST TEXT", TextStyle::default()),
                text_anchor: Default::default(),
                text_2d_bounds: Default::default(),
                transform: Transform::from_xyz(0.0, 20.0, 10.0),
                global_transform: Default::default(),
                visibility: Default::default(),
                inherited_visibility: Default::default(),
                view_visibility: Default::default(),
                text_layout_info: Default::default(),
            });
        });
    }*/
}

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), add_people);
    }
}
