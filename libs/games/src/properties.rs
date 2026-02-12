use bevy::render::view::screenshot::ScreenshotCaptured;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    // setup loading screen
    // Begin, 
    Defeat,

    #[default]
    LoadingScreen,
    // via asset_collections
    LoadingAssets,
    // other useful systems ?
    // Loading,

    PacmanEnter,
    FlappyBird,
    Geometry,
    Platformer,
    Miami,
    Titles,
    Novel,
    Fnaf,
    FakeEnd,
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


#[derive(Resource, Default)]
pub struct LastScreenshot {
    pub image: Option<Handle<Image>>,
    pub awaiting: bool
}

pub fn await_screenshot_and_translate(
    state: AppState
) -> impl FnMut(On<ScreenshotCaptured>, ResMut<LastScreenshot>, ResMut<Assets<Image>>, ResMut<NextState<AppState>>) {
    move |
            screenshot_captured,
            mut last_screenshot,
            mut images_mut,
            mut next_state
    | {
        let img = screenshot_captured.image.clone();
        let handle = images_mut.add(img);
        last_screenshot.image = Some(handle.clone());
        next_state.set(state);
        last_screenshot.awaiting = false;
    }
}

pub const MUSIC_INTERPOLATION : f32 = 0.6;

pub const PACMAN_EAT_PATH_HALF : f32 = 280.0;
pub const PACMAN_EAT_SCALE : f32 = 4.0;
pub const PACMAN_EAT_ANIM_DELAY : f32 = 0.06;
pub const PACMAN_EAT_WALK_SPEED : f32 = 400.0;



pub const PLATFORMER_GRAVITY_FORCE : f32 = 50.0;
pub const PLATFORMER_JUMP_FORCE : f32 = 300.0;
pub const PLATFORMER_MAX_SPEED : f32 = 200.0;
pub const PLATFORMER_AIR_GAIN : f32 = 1000.0;
pub const PLATFORMER_GROUND_GAIN : f32 = 1500.0;
pub const PLATFORMER_ANIM_DELAY : f32 = 0.1;



pub const FLAPPY_WIDTH : f32 = 576.0;
pub const FLAPPY_HALF_HEIGHT : f32 = 260.0 / 2.0;
pub const FLAPPY_SCALE : f32 = 1.0 / 16.0;
pub const FLAPPY_LEFT_BOUND : f32 = -FLAPPY_WIDTH / 2.0 + 120.0;
pub const FLAPPY_RIGHT_BOUND : f32 = FLAPPY_WIDTH / 2.0 - 200.0;

pub const FLAPPY_TRANSITION_SPEED : f32 = 500.;
pub const FLAPPY_BG_TRANSITION_SPEED : f32 = 800.;
pub const FLAPPY_PARALLAX_SPEED : f32 = 50.0;
pub const FLAPPY_BIRD_PROGRESS_SPEED : f32 = 25.0;
pub const FLAPPY_BIRD_OUT_SPEED : f32 = 500.0;
pub const FLAPPY_PIPE_SPEED : f32 = 250.0;
pub const FLAPPY_PIPE_SPAWN_DELAY : f32 = 1.5;
pub const FLAPPY_PIPE_GAP : f32 = 85.0;
pub const FLAPPY_PIPE_SPREAD : f32 = 40.0;
pub const FLAPPY_GRAVITY_AFFECT : f32 = 50.0;
pub const FLAPPY_BIRD_JUMP_STRENGTH : f32 = 200.0;
pub const FLAPPY_DEATH_DELAY : f32 = 1.0;
pub const FLAPPY_PIPE_SCALE : f32 = 0.25;

pub const NOVEL_MUSIC_INTERPOLATION : f32 = 1.0;
