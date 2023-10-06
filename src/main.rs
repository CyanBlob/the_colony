use crate::character_plugin::CharacterPlugin;
use crate::name_plugin::NamePlugin;
use crate::world_gen_plugin::WorldGenPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_pancam::{PanCam, PanCamPlugin};

mod character_plugin;
mod name_plugin;
mod world_gen_plugin;

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
    #[asset(path = "terrain/ugly_flower.png")]
    ugly_flower: Handle<Image>,
}

fn main() {
    App::new()
        /*.add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(handle_ui_buttons)
        )
        // setup when entering the state
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_menu)
        )
        // cleanup when exiting the state
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(close_menu)
        )*/
        .add_state::<crate::AppState>()
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame))
        .add_collection_to_loading_state::<_, MyAssets>(AppState::Loading)
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            CharacterPlugin,
            NamePlugin,
            WorldGenPlugin,
            PanCamPlugin::default(),
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
