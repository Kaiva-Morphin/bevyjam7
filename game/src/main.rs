use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;

use crate::dev_games::*;
use crate::dev_games::plugin::*;
use crate::prelude::*;

use crate::core::plugin::CorePlugin;
pub mod prelude;
pub mod properties;
pub mod dev_games;
// pub mod pathfinder;
pub mod novel2fnaf;

fn main() {
    App::new()
        .add_plugins((
            CorePlugin::default(),
            GamesPlugin
        ))
        .run();
}


struct GamesPlugin;
#[cfg(feature="yaro")]
impl Plugin for GamesPlugin {
    fn build(&self, app: &mut App) {
        use bevy::asset::embedded_asset;

        use crate::{hints::{HintAssets, update_hints}, pathfinder::plugin::PathfinderPlugin};
        let omit_prefix = "";
        embedded_asset!(app, omit_prefix, "../assets/images/loading_screen.jpg");

        app
            .insert_resource(LastState::default())
            .insert_resource(LastScreenshot::default())
            .init_state::<AppState>()
            .add_systems(Update, update_hints)
            .add_systems(Startup, setup_loading_screen)
            .add_systems(OnExit(AppState::LoadingAssets), cleanup_loading_screen)
            .add_loading_state(
                LoadingState::new(AppState::LoadingAssets)
                    .continue_to_state(AppState::Novel)
                    .load_collection::<GameAssets>()
                    .load_collection::<HintAssets>()
                    .load_collection::<pacman_eat::plugin::PacmanEatAssets>()
                    .load_collection::<flappy_bird::plugin::FlappyBirdAssets>()
                    .load_collection::<platformer::plugin::PlatformerAssets>()
                    .load_collection::<novel::plugin::ActorsAssets>()
                    .load_collection::<novel::plugin::BackgroundsAssets>()
                    .load_collection::<novel::plugin::NovelAssets>()
                    .load_collection::<novel::plugin::NovelMusicAssets>()
                    .load_collection::<novel::plugin::NovelSoundEffectsAssets>()
                    .load_collection::<fake_end::plugin::FakeEndAssets>()
                    .load_collection::<fnaf::plugin::FNAFAssets>()
                    .load_collection::<miami::plugin::MiamiAssets>()
                    .load_collection::<geometry_dash::plugin::GeometryDashAssets>()
            )
            .add_plugins((
                PathfinderPlugin,
                pacman_eat::plugin::PacmanEatPlugin,
                geometry_dash::plugin::GeometryDashPlugin,
                flappy_bird::plugin::FlappyBirdPlugin,
                platformer::plugin::PlatformerPlugin,
                novel::plugin::NovelPlugin,
                fake_end::plugin::FakeEndPlugin,
                fnaf::plugin::FNAFPlugin,
                novel2fnaf::plugin::Novel2FnafPlugin,
                miami::plugin::MiamiPlugin,
            ))
            .add_systems(Startup, warmup_screenshot)
            .add_systems(OnEnter(AppState::Defeat), on_defeat)
            .add_systems(Update, (
                bevy::dev_tools::states::log_transitions::<AppState>,
                animate_screenshot
            ))
        ;
    }
}



#[cfg(not(feature = "yaro"))]
impl Plugin for GamesPlugin {
    fn build(&self, app: &mut App) {
        use bevy::asset::embedded_asset;

        use crate::{hints::{HintAssets, update_hints}, pathfinder::plugin::PathfinderPlugin};
        let omit_prefix = "";
        embedded_asset!(app, omit_prefix, "../assets/images/loading_screen.jpg");

        app
            .insert_resource(LastState::default())
            .insert_resource(LastScreenshot::default())
            .init_state::<AppState>()
            .add_systems(Update, update_hints)
            .add_systems(Startup, setup_loading_screen)
            .add_systems(OnExit(AppState::LoadingAssets), cleanup_loading_screen)
            .add_loading_state(
                LoadingState::new(AppState::LoadingAssets)
                    .continue_to_state(AppState::Miami)
                    .load_collection::<GameAssets>()
                    .load_collection::<HintAssets>()
                    .load_collection::<pacman_eat::plugin::PacmanEatAssets>()
                    .load_collection::<flappy_bird::plugin::FlappyBirdAssets>()
                    .load_collection::<platformer::plugin::PlatformerAssets>()
                    .load_collection::<novel::plugin::ActorsAssets>()
                    .load_collection::<novel::plugin::BackgroundsAssets>()
                    .load_collection::<novel::plugin::NovelAssets>()
                    .load_collection::<novel::plugin::NovelMusicAssets>()
                    .load_collection::<novel::plugin::NovelSoundEffectsAssets>()
                    .load_collection::<fake_end::plugin::FakeEndAssets>()
                    .load_collection::<fnaf::plugin::FNAFAssets>()
                    .load_collection::<miami::plugin::MiamiAssets>()
                    .load_collection::<geometry_dash::plugin::GeometryDashAssets>()
            )
            .add_plugins((
                PathfinderPlugin,
                pacman_eat::plugin::PacmanEatPlugin,
                geometry_dash::plugin::GeometryDashPlugin,
                flappy_bird::plugin::FlappyBirdPlugin,
                platformer::plugin::PlatformerPlugin,
                novel::plugin::NovelPlugin,
                fake_end::plugin::FakeEndPlugin,
                fnaf::plugin::FNAFPlugin,
                novel2fnaf::plugin::Novel2FnafPlugin,
                miami::plugin::MiamiPlugin,
            ))
            .add_systems(Startup, warmup_screenshot)
            .add_systems(OnEnter(AppState::Defeat), on_defeat)
            .add_systems(Update, (
                bevy::dev_tools::states::log_transitions::<AppState>,
                animate_screenshot
            ))
        ;
    }
}

#[derive(Component)]
struct LoadingScreen;

fn setup_loading_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppState>>,
) {
    let path = std::path::Path::new("game").join("../assets/images/loading_screen.jpg");
    let source = bevy::asset::io::AssetSourceId::from("embedded");
    let asset_path = bevy::asset::AssetPath::from_path(&path).with_source(source);

    commands.spawn((Sprite::from_image(asset_server.load(asset_path)), LoadingScreen));
    state.set(AppState::LoadingAssets);
}

fn cleanup_loading_screen(
    mut commands: Commands,
    q: Query<Entity, With<LoadingScreen>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}