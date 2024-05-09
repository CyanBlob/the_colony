use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_debug_text_overlay::OverlayPlugin;
use bevy_enum_filter::prelude::*;
use bevy_fast_tilemap::FastTileMapPlugin;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_pancam::{PanCam, PanCamPlugin};
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};

use crate::character_plugin::CharacterPlugin;
use crate::debug_plugin::DebugPlugin;
use crate::growth_plugin::PlanGrowthPlugin;
use crate::input_plugin::InputPlugin;
use crate::name_plugin::NamePlugin;
use crate::task_scorer::TaskScoringPlugin;
use crate::tasks::{AllTasks, BasicTasksPlugin};
#[allow(unused)]
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::wander_plugin::RandomMovementPlugin;
use crate::world_gen_plugin::WorldGenPlugin;

mod character_plugin;
mod debug_plugin;
mod growth_plugin;
mod name_plugin;
mod pathing;
mod task_scorer;
mod tasks;
mod wander_plugin;
mod world_gen_plugin;
mod input_plugin;

#[allow(unused)]
#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    Loading,
    MainMenu,
    CreateWorld,
    InGame,
    Paused,
}


#[derive(Resource, Default)]
struct TerrainFolder(Handle<LoadedFolder>);

#[derive(Resource, Default)]
#[allow(unused)]
struct PlantFolder(Handle<LoadedFolder>);

#[derive(Resource, Default)]
struct CharacterFolder(Handle<LoadedFolder>);

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    commands.insert_resource(CharacterFolder(asset_server.load_folder("characters")));
    commands.insert_resource(TerrainFolder(asset_server.load_folder("terrain")));
    commands.insert_resource(PlantFolder(asset_server.load_folder("plants")));
}

#[allow(unused)]
fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    terrain_sprite_folder: Res<TerrainFolder>,
    plant_sprite_folder: Res<PlantFolder>,
    character_folder: Res<CharacterFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // TODO: Ensure characters folder is also loaded
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&terrain_sprite_folder.0) {
            next_state.set(AppState::CreateWorld);
        }
    }
}

#[allow(unused)]
#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "terrain/ugly_grass.png")]
    ugly_grass: Handle<Image>,
    #[asset(path = "terrain/ugly_grass2.png")]
    ugly_grass2: Handle<Image>,
    #[asset(path = "terrain/ugly_grass3.png")]
    ugly_grass3: Handle<Image>,
    #[asset(path = "terrain/ugly_grass4.png")]
    ugly_grass4: Handle<Image>,

    #[asset(path = "terrain/ugly_mud.png")]
    ugly_mud: Handle<Image>,
    #[asset(path = "terrain/ugly_mud2.png")]
    ugly_mud2: Handle<Image>,
    #[asset(path = "terrain/ugly_mud3.png")]
    ugly_mud3: Handle<Image>,
    #[asset(path = "terrain/ugly_mud4.png")]
    ugly_mud4: Handle<Image>,

    #[asset(path = "plants/ugly_flower.png")]
    ugly_flower: Handle<Image>,

    #[asset(path = "characters/character.png")]
    character: Handle<Image>,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        //.add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame))
        //.add_collection_to_loading_state::<_, MyAssets>(AppState::Loading)
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            CharacterPlugin,
            NamePlugin,
            WorldGenPlugin,
            PlanGrowthPlugin,
            PanCamPlugin::default(),
            RandomMovementPlugin,
            TaskScoringPlugin,
            BasicTasksPlugin,
            OverlayPlugin {
                font_size: 14.0,
                ..default()
            },
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            //TilemapPlugin,
            //ThirstPlugin,
            //WorldInspectorPlugin::new(),
        ))
        .add_plugins(InputPlugin)
        .add_plugins((DebugPlugin, bevy_framepace::FramepacePlugin, PerfUiPlugin, FastTileMapPlugin::default(), ))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Loading), load_textures)
        .add_systems(Update, check_textures.run_if(in_state(AppState::Loading)))
        .add_enum_filter::<AllTasks>()
        .run();
}

fn setup(mut commands: Commands, mut framepace: ResMut<FramepaceSettings>) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        min_scale: 0.1,
        max_scale: Some(30.0),
        ..default()
    });

    framepace.limiter = Limiter::Off;

    commands.spawn(PerfUiCompleteBundle::default());

    /*commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryClock::default(),
    ));*/
}