use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;

use crate::dev_games::*;
use crate::dev_games::plugin::*;
use crate::prelude::*;

use crate::core::plugin::CorePlugin;
pub mod prelude;
pub mod properties;
pub mod dev_games;


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
        app
            .insert_resource(LastState::default())
            .insert_resource(LastScreenshot::default())
            .init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::LoadingAssets)
                    .continue_to_state(AppState::Fnaf)
                    .load_collection::<GameAssets>()
                    .load_collection::<pacman_eat::plugin::PacmanEatAssets>()
                    .load_collection::<flappy_bird::plugin::FlappyBirdAssets>()
                    .load_collection::<platformer::plugin::PlatformerAssets>()
                    .load_collection::<novel::plugin::ActorsAssets>()
                    .load_collection::<novel::plugin::BackgroundsAssets>()
                    .load_collection::<novel::plugin::NovelAssets>()
                    .load_collection::<fake_end::plugin::FakeEndAssets>()
                    .load_collection::<fnaf::plugin::FNAFAssets>()
            )
            .add_plugins((
                pacman_eat::plugin::PacmanEatPlugin,
                flappy_bird::plugin::FlappyBirdPlugin,
                platformer::plugin::PlatformerPlugin,
                novel::plugin::NovelPlugin,
                fake_end::plugin::FakeEndPlugin,
                fnaf::plugin::FNAFPlugin,
            ))
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
        app
            .insert_resource(LastState::default())
            .insert_resource(LastScreenshot::default())
            .init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::LoadingAssets)
                    .continue_to_state(AppState::Miami)
                    .load_collection::<GameAssets>()
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
            )
            .add_plugins((
                pacman_eat::plugin::PacmanEatPlugin,
                flappy_bird::plugin::FlappyBirdPlugin,
                platformer::plugin::PlatformerPlugin,
                novel::plugin::NovelPlugin,
                fake_end::plugin::FakeEndPlugin,
                fnaf::plugin::FNAFPlugin,
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
