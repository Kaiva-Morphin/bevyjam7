use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;

use crate::games::*;
use crate::games::plugin::*;
use crate::prelude::*;

use crate::core::plugin::CorePlugin;
pub mod character;
pub mod core;
pub mod tilemap;
pub mod prelude;
pub mod properties;
pub mod games;

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
            .init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::LoadingAssets)
                    .continue_to_state(AppState::Geometry)
                    .load_collection::<pacman_eat::plugin::PacmanEatAssets>()
                    .load_collection::<flappy_bird::plugin::FlappyBirdAssets>()
                    .load_collection::<geometry_dash::plugin::GeometryDashAssets>()
            )
            .add_plugins((
                pacman_eat::plugin::PacmanEatPlugin,
                flappy_bird::plugin::FlappyBirdPlugin,
                geometry_dash::plugin::GeometryDashPlugin,
            ))
            .add_systems(OnEnter(AppState::Defeat), on_defeat)
            .add_systems(Update, bevy::dev_tools::states::log_transitions::<AppState>)
        ;
    }
}


#[cfg(feature="kaiv")]
impl Plugin for GamesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LastState::default())
            .init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::LoadingAssets)
                    .continue_to_state(AppState::Platformer)
                    .load_collection::<pacman_eat::plugin::PacmanEatAssets>()
                    .load_collection::<flappy_bird::plugin::FlappyBirdAssets>()
                    .load_collection::<platformer::plugin::PlatformerAssets>()
            )
            .add_plugins((
                pacman_eat::plugin::PacmanEatPlugin,
                flappy_bird::plugin::FlappyBirdPlugin,
                platformer::plugin::PlatformerPlugin,
            ))
            .add_systems(OnEnter(AppState::Defeat), on_defeat)
            .add_systems(Update, bevy::dev_tools::states::log_transitions::<AppState>)
        ;
    }
}

#[cfg(not(feature = "kaiv"))]
#[cfg(not(feature = "yaro"))]
impl Plugin for GamesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LastState::default())
            .init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::LoadingAssets)
                    .continue_to_state(AppState::Novel)
                    .load_collection::<pacman_eat::plugin::PacmanEatAssets>()
                    .load_collection::<flappy_bird::plugin::FlappyBirdAssets>()
                    .load_collection::<platformer::plugin::PlatformerAssets>()
                    .load_collection::<novel::plugin::ActorsAssets>()
                    .load_collection::<novel::plugin::BackgroundsAssets>()
                    .load_collection::<novel::plugin::NovelAssets>()
            )
            .add_plugins((
                pacman_eat::plugin::PacmanEatPlugin,
                flappy_bird::plugin::FlappyBirdPlugin,
                platformer::plugin::PlatformerPlugin,
                novel::plugin::NovelPlugin,
            ))
            .add_systems(OnEnter(AppState::Defeat), on_defeat)
            .add_systems(Update, bevy::dev_tools::states::log_transitions::<AppState>)
        ;
    }
}