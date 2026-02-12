use bevy::color::palettes::css::RED;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;

pub struct FNAFPlugin;

const STATE: AppState = AppState::Fnaf;
const NEXT_STATE: AppState = AppState::PacmanEnter;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = STATE)]
#[states(scoped_entities)]
enum LocalState {
    #[default]
    InitialAnim,
    Game,
    Defeat,
    Win,
}

#[derive(AssetCollection, Resource)]
pub struct FNAFAssets {
    #[asset(path = "images/fnaf/room.png")]
    room: Handle<Image>,
    #[asset(path = "images/fnaf/white.png")]
    white_button: Handle<Image>,
    #[asset(path = "images/fnaf/red.png")]
    red_button: Handle<Image>,
    #[asset(path = "images/fnaf/door.png")]
    door: Handle<Image>,
    #[asset(path = "images/fnaf/window.png")]
    window: Handle<Image>,
}

impl Plugin for FNAFPlugin {
    fn build(&self, app: &mut App) {
        app
            // .register_type::<CameraCenter>()
            // .register_type::<SpawnPoint>()
            .add_sub_state::<LocalState>()
            // .add_observer(spawnpoint_handler)
            // .add_observer(camera_handler)
            // .add_observer(on_collider_spawned)
            .add_systems(OnEnter(STATE), (setup, init_rects))
            .add_systems(Update, tick_transition.run_if(in_state(LocalState::InitialAnim)))
            // .add_systems(OnEnter(LocalState::Game), begin_game)
            .add_systems(Update, (update_mouse_pos).run_if(in_state(LocalState::Game)))
            // .add_systems(Update, tick_defeat.run_if(in_state(LocalState::Defeat)))
            // .add_systems(Update, tick_win.run_if(in_state(LocalState::Win)))
            .add_systems(OnExit(STATE), cleanup)
            ;
    }
}

#[derive(Resource)]
pub struct MousePos(pub Option<Vec2>);

#[derive(Component)]
pub struct DbgSprite;

#[derive(Resource)]
pub struct Rects {
    pub red_right: Rect,
    pub red_left: Rect,
    pub white_right: Rect,
    pub white_left: Rect,
    pub faz: Rect,
}

// 0 0 - top left; LR 53 528, 121 410; LW 52 673, 122 570; faz 713 386, 730 373
// 1728x972

const SPRITE_XSIZE: f32 = 1728.;
const SPRITE_YSIZE: f32 = 972.;

fn init_rects(
    mut cmd: Commands,
) {
    let rects = Rects {
        red_left: Rect::from_corners(
            Vec2::new(-809.5165, -36.752216),
            Vec2::new(-744.71643, 67.70151),
        ),
        red_right: Rect::from_corners(
            Vec2::new(749.5523, -35.7851),
            Vec2::new(813.3852, 75.4388),
        ),
        white_left: Rect::from_corners(
            Vec2::new(-807.5821, -185.69551),
            Vec2::new(-745.68365, -93.814964),
        ),
        white_right: Rect::from_corners(
            Vec2::new(750.5195, -180.85966),
            Vec2::new(803.7135, -90.91342),
        ),
        faz: Rect::from_corners(
            Vec2::new(-156.68054, 95.74923),
            Vec2::new(-127.665634, 116.0597),
        ),
    };
    cmd.insert_resource(rects);
}

fn update_mouse_pos(
    window: Single<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    canvas: Res<camera::ViewportCanvas>,
    mut pos: ResMut<MousePos>,
) {
    let window = *window;
    let Some(cursor_win) = window.cursor_position() else { return; }; // top-left origin (Bevy >= 0.11)
    let (camera, cam_transform) = match camera_q.single() {
        Ok(v) => v,
        Err(_) => return,
    };

    let image_size = canvas.size;          // Vec2: image pixel size (physical/logical as you track it)
    let window_size = canvas.window_size;  // Vec2: window size used during resize

    // compute top-left offset where the image is blitted in the window
    let offset = (window_size - image_size) * 0.5;

    let local = cursor_win - offset;

    let viewport_pos = if let Some(ur) = camera.physical_viewport_rect() {
        let min = Vec2::new(ur.min.x as f32, ur.min.y as f32);
        local - min
    } else {
        local
    };

    match camera.viewport_to_world_2d(cam_transform, viewport_pos) {
        Ok(world_pos) => {
            info!("cursor world pos: {:?}", world_pos);
            pos.0 = Some(world_pos)
        }
        Err(err) => {
            warn!("viewport_to_world_2d failed: {:?}", err);
        }
    }
}

fn handle_rects(
    rects: Res<Rects>,
    mouse_pos: Res<MousePos>,
    gizmos: Gizmos,
) {
    for rect in rects {
        gizmos.rect_2d(rect.center(), rect.size(), Color::Srgba(RED));
    }
}

fn tick_transition(
    mut state: ResMut<NextState<LocalState>>,
) {
    state.set(LocalState::Game);
}

#[derive(Component)]
pub enum Environment {
    Room,
    LDoor,
    RDoor,
    LWhitelight,
    RWhitelight,
    LRedlight,
    RRedlight,
    LWindow,
    RWindow,
    Faz,
}

fn setup(
    mut cmd: Commands,
    fnaf_assets: Res<FNAFAssets>,
    mut proj: Query<&mut Projection, With<WorldCamera>>
) {
    match &mut *proj.single_mut().expect("nocam") {
        Projection::Orthographic(proj) => {
            proj.scale = 3.0;
        },
        _ => {}
    }
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.room.clone(),
            ..default()
        },
        Environment::Room,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.door.clone(),
            ..default()
        },
        Environment::LDoor,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.door.clone(),
            flip_x: true,
            ..default()
        },
        Environment::RDoor,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.white_button.clone(),
            ..default()
        },
        Environment::LWhitelight,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.white_button.clone(),
            flip_x: true,
            ..default()
        },
        Environment::RWhitelight,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.red_button.clone(),
            ..default()
        },
        Environment::LRedlight,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.red_button.clone(),
            flip_x: true,
            ..default()
        },
        Environment::RRedlight,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.red_button.clone(),
            ..default()
        },
        Environment::LWindow,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.red_button.clone(),
            flip_x: true,
            ..default()
        },
        Environment::RWindow,
    ));
}

fn cleanup(
    mut cmd: Commands,
    mut proj: Query<&mut Projection, With<WorldCamera>>
) {
    match &mut *proj.single_mut().expect("nocam") {
        Projection::Orthographic(proj) => {
            proj.scale = 0.8;
        },
        _ => {}
    }
    cmd.remove_resource::<MousePos>();
    cmd.remove_resource::<Rects>();
}