use crate::character_plugin::CharacterPlugin;
use crate::growth_plugin::PlanGrowthPlugin;
use crate::name_plugin::NamePlugin;
use crate::world_gen_plugin::WorldGenPlugin;
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use bevy::tasks::Task;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_debug_text_overlay::OverlayPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use crate::task_scorer::TaskScoringPlugin;
use bevy_enum_filter::prelude::*;
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin, PerfUiRoot};
use iyes_perf_ui::prelude::{PerfUiEntryClock, PerfUiEntryFPS};

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
        .init_state::<AppState>()
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame))
        .add_collection_to_loading_state::<_, MyAssets>(AppState::Loading)
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
            OverlayPlugin { font_size: 14.0, ..default()
},
            PerfUiPlugin,
                bevy::diagnostic::FrameTimeDiagnosticsPlugin,
                bevy::diagnostic::EntityCountDiagnosticsPlugin,
                bevy::diagnostic::SystemInformationDiagnosticsPlugin,
                                                      //ThirstPlugin,
                                                      //WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_enum_filter::<AllTasks>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        min_scale: 0.1,
        max_scale: Some(30.0),
        ..default()
    });

    commands.spawn(PerfUiCompleteBundle::default());
    /*commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryClock::default(),
    ));*/
}

// Create a texture atlas with the given padding and sampling settings
// from the individual sprites in the given folder.
/*fn create_texture_atlas(
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
}*/
