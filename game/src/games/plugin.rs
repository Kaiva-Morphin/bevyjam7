use std::time::Duration;

use bevy::render::view::screenshot::ScreenshotCaptured;
use bevy_asset_loader::asset_collection::AssetCollection;
use camera::ViewportCanvas;

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
    FlappyBird, // 60%
    Geometry, // 90%
    Platformer, // 90% EXTEND?
    Hotline,
    Titles,
    Novel, // 90% 
    
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

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "images/hand.png")]
    pub hand: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 402, tile_size_y = 533, columns = 3, rows = 1))]
    pub hand_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Default)]
pub struct ScreenshotEntity {
    t: f32,
}

#[derive(Component, Default)]
pub struct HandEntity {}

pub fn on_defeat(
    res: Res<LastState>,
    mut state: ResMut<NextState<AppState>>,
    mut cmd: Commands,
    last_screenshot: ResMut<LastScreenshot>,
    canvas: Res<camera::ViewportCanvas>,
    h_q: Query<Entity, With<HandEntity>>,
    assets: Res<GameAssets>,
    cam: Query<Entity, With<WorldCamera>>,
    ui_scale: Res<UiScale>,
    // canvas: Res<camera::ViewportCanvas>,
) {
    for e in h_q.iter() {cmd.entity(e).despawn();}
    let Some(new_handle) = last_screenshot.image.clone() else {
        return;
    };
    // cmd.spawn((
    //     Name::new("Screenshot"),
    //     Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)).with_scale(Vec3::splat(1.0)),
    //     Sprite {
    //         image: new_handle,
    //         ..default()
    //     },
    //     ScreenshotEntity::default(),
    // ));
    if! h_q.is_empty() {return;}
    let tween1 = Tween::new(
        // Use a quadratic easing on both endpoints.
        // EaseFunction::CircularOut,
        EaseFunction::SineOut,
        // Animation time (one way only; for ping-pong it takes 2 seconds
        // to come back to start).
        Duration::from_secs_f32(HAND_IN_ANIMATION_DURATION),
        // The lens gives the TweenAnimator access to the Transform component,
        // to animate it. It also contains the start and end values associated
        // with the animation ratios 0. and 1.
        UiPositionLens {
            start: UiRect {
                left: Val::Px(300.0),
                top: Val::Px(300.0),
                ..Default::default()
            },
            end: UiRect {
                left: Val::Px(175.0),
                top: Val::Px(10.0),
                ..Default::default()
            },
        },
    );
    let dummy = Tween::new(
        // Use a quadratic easing on both endpoints.
        // EaseFunction::CircularOut,
        EaseFunction::SineOut,
        // Animation time (one way only; for ping-pong it takes 2 seconds
        // to come back to start).
        Duration::from_secs_f32(HAND_IN_ANIMATION_DURATION),
        // The lens gives the TweenAnimator access to the Transform component,
        // to animate it. It also contains the start and end values associated
        // with the animation ratios 0. and 1.
        UiPositionLens {
            start: UiRect {
                ..Default::default()
            },
            end: UiRect {
                ..Default::default()
            },
        },
    );

    let tween2 = Tween::new(
        // Use a quadratic easing on both endpoints.
        // EaseFunction::CircularOut,
        EaseFunction::SineIn,
        // Animation time (one way only; for ping-pong it takes 2 seconds
        // to come back to start).
        Duration::from_secs_f32(HAND_OUT_ANIMATION_DURATION),
        // The lens gives the TweenAnimator access to the Transform component,
        // to animate it. It also contains the start and end values associated
        // with the animation ratios 0. and 1.
        UiPositionLens {
            start: UiRect {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..Default::default()
            },
            end: UiRect {
                left: Val::Px(230.0),
                top: Val::Px(400.0),
                ..Default::default()
            }
        },
    );
    info!("Spawning hand");
    let cam = cam.iter().next().expect("No cam!");

    let screen = cmd.spawn((
        UiTargetCamera(cam),
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode {
            image: new_handle,
            ..default()
        },
        ScreenshotEntity::default(),
        TweenAnim::new(dummy.then(tween2)),
        children![(
            Name::new("Hand"),
            Node{
                ..default()
            },
            ImageNode {
                image: assets.hand.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.hand_layout.clone(),
                    ..default()
                }),
                ..default()
            },
            HandEntity::default(),
            TweenAnim::new(tween1),
        )],
    ));
    state.set(res.state);
}

pub fn animate_screenshot(
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScreenshotEntity)>,
    mut h_q: Query<(Entity, &mut ImageNode), With<HandEntity>>,
    mut cmd: Commands
){
    let dt = time.dt();
    for (e, mut s) in query.iter_mut() {
        s.t += dt;
        if s.t > HAND_IN_ANIMATION_DURATION {
            for (_, mut i) in h_q.iter_mut() {
                if let Some(t) = &mut i.texture_atlas {
                    t.index = 2;
                }
            }
        }
        if s.t > HAND_IN_ANIMATION_DURATION + HAND_OUT_ANIMATION_DURATION {
            // info!("Despawning hand");
            s.t = 0.0;
            cmd.entity(e).despawn();
        }
    }
}

