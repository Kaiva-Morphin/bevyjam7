use std::time::Duration;

use bevy::render::view::screenshot::Screenshot;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;


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




pub fn warmup_screenshot(
    mut cmd: Commands
) {
    cmd.spawn(Screenshot::primary_window());
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
    // cam: Query<Entity, With<WorldCamera>>,
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
        TransformPositionLens {
            start: vec3(1.0, -1.0, 2.0) * canvas.window_size.extend(0.0) * 0.75,
            end: vec3(0., 0., 2.,),
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
        TransformPositionLens {
            start: vec3(0., 0., 0.,),
            end: vec3(0., 0., 0.,),
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
        TransformPositionLens {
            start: vec3(0., 0., 0.,),
            end: vec3(0.5, -1.0, 0.0) * canvas.window_size.extend(0.0) * 1.5,
        },
    );
    // let cam = cam.iter().next().expect("No cam!");
    cmd.spawn((
        Sprite {
            image: new_handle,
            ..default()
        },
        ScreenshotEntity::default(),
        HIGHRES_LAYERS,
        TweenAnim::new(dummy.then(tween2)),
        children![(
            HIGHRES_LAYERS,
            Name::new("Hand"),
            Transform::default().with_scale(Vec3::splat(ui_scale.0 * 0.5)),
            Sprite {
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

    // cmd.spawn((
    //     UiTargetCamera(cam),
    //     Node{
    //         width: Val::Percent(100.0),
    //         height: Val::Percent(100.0),
    //         ..default()
    //     },
    //     ImageNode {
    //         image: new_handle,
    //         ..default()
    //     },
    //     ScreenshotEntity::default(),
    //     TweenAnim::new(dummy.then(tween2)),
    //     children![(
    //         Name::new("Hand"),
    //         Node{
    //             ..default()
    //         },
    //         ImageNode {
    //             image: assets.hand.clone(),
    //             texture_atlas: Some(TextureAtlas {
    //                 layout: assets.hand_layout.clone(),
    //                 ..default()
    //             }),
    //             ..default()
    //         },
    //         HandEntity::default(),
    //         TweenAnim::new(tween1),
    //     )],
    // ));
    state.set(res.state);
}

pub fn animate_screenshot(
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScreenshotEntity)>,
    mut h_q: Query<(Entity, &mut Sprite), With<HandEntity>>,
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
            s.t = 0.0;
            cmd.entity(e).despawn();
        }
    }
}

