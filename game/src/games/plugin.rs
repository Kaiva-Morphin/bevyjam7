use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    // setup loading screen
    // Begin, 
    Defeat,

    #[default]
    // via asset_collections
    LoadingAssets,
    // other useful systems ?
    // Loading,

    PacmanEnter, // 95%
    FlappyBird, // 30%
    Geometry,
    Platformer, // 90% EXTEND?
    Hotline,
    Titles,
    Novel,
    
    End
}

#[derive(Resource)]
pub struct LastState {
    pub state: AppState,
    pub screenshot: Option<Handle<Image>>
}

impl Default for LastState {
    fn default() -> Self {
        Self {
            state: AppState::PacmanEnter,
            screenshot: None
        }
    }
}

pub struct GamesPlugin;

// THIS IS GLOBAL PLUGIN. WE NEED TO USE OUR CUSTOM
// impl Plugin for GamesPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .init_state::<AppState>()
//             .insert_resource(LastState::default())
//             .add_loading_state(
//                 LoadingState::new(AppState::LoadingAssets)
//                     .continue_to_state(AppState::FlappyBird)
//                     .load_collection::<super::pacman_eat::plugin::PacmanEatAssets>()
//                     .load_collection::<super::flappy_bird::plugin::FlappyBirdAssets>()
//             )
//             .add_plugins((
//                 super::pacman_eat::plugin::PacmanEatPlugin,
//                 super::flappy_bird::plugin::FlappyBirdPlugin,
//             ))
//             .add_systems(OnEnter(AppState::Defeat), on_defeat)
//             .add_systems(Update, bevy::dev_tools::states::log_transitions::<AppState>)
//            ;
//     }
// }

pub fn on_defeat (
    res : Res<LastState>,
    mut state: ResMut<NextState<AppState>>
) {
    // todo! screenshot
    state.set(res.state);
}