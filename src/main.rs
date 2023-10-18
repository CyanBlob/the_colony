use crate::character_plugin::CharacterPlugin;
use crate::growth_plugin::PlanGrowthPlugin;
use crate::name_plugin::NamePlugin;
use crate::world_gen_plugin::WorldGenPlugin;
use bevy::prelude::*;
use bevy::tasks::Task;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_enum_filter::prelude::AddEnumFilter;
use bevy_pancam::{PanCam, PanCamPlugin};
use crate::task_scorer::TaskScoringPlugin;

#[allow(unused)]
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::wander_plugin::RandomMovementPlugin;
use crate::tasks::{AllTasks, BasicTasksPlugin};

mod character_plugin;
mod growth_plugin;
mod name_plugin;
mod wander_plugin;
mod world_gen_plugin;
mod task_scorer;
mod tasks;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
}

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

    #[asset(path = "terrain/ugly_flower.png")]
    ugly_flower: Handle<Image>,

    #[asset(path = "characters/character.png")]
    character: Handle<Image>,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame))
        .add_collection_to_loading_state::<_, MyAssets>(AppState::Loading)
        .add_enum_filter::<AllTasks>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CharacterPlugin,
            NamePlugin,
            WorldGenPlugin,
            PlanGrowthPlugin,
            PanCamPlugin::default(),
            RandomMovementPlugin,
            TaskScoringPlugin,
            BasicTasksPlugin,
            bevy_screen_diags::ScreenDiagsTextPlugin, // TODO: debug only
                                                      //ThirstPlugin,
                                                      //WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        min_scale: 0.1,
        max_scale: Some(3.0),
        ..default()
    });
}
