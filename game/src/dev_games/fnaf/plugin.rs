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
            Vec2::new(53., SPRITE_YSIZE - 528.),
            Vec2::new(121., SPRITE_YSIZE - 410.),
        ),
        red_right: Rect::from_corners(
            Vec2::new(SPRITE_XSIZE - 53., SPRITE_YSIZE - 528.),
            Vec2::new(SPRITE_XSIZE - 121., SPRITE_YSIZE - 410.),
        ),
        white_left: Rect::from_corners(
            Vec2::new(52., SPRITE_YSIZE - 673.),
            Vec2::new(122., SPRITE_YSIZE - 570.),
        ),
        white_right: Rect::from_corners(
            Vec2::new(SPRITE_XSIZE - 52., SPRITE_YSIZE - 673.),
            Vec2::new(SPRITE_XSIZE - 122., SPRITE_YSIZE - 570.),
        ),
        faz: Rect::from_corners(
            Vec2::new(713., SPRITE_YSIZE - 386.),
            Vec2::new(730., SPRITE_YSIZE - 373.),
        ),
    };
    cmd.insert_resource(rects);
}

fn update_mouse_pos(
    window: Single<&Window>,
    outer_camera_q: Single<(&Camera, &GlobalTransform), With<HighresCamera>,>,
    world_camera_q: Single<&GlobalTransform, With<WorldCamera>,>,
    camera_q: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    mut sp: Local<Option<Entity>>,
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    ui: Res<UiScale>,
    canvas: Res<camera::ViewportCanvas>,
) {
    let Some(e) = *sp else {
        info!("Spawned!");
        *sp = Some(cmd.spawn((
            Name::new("ABOBA"),
            Sprite {
                image: asset_server.load("placeholder.jpg"),
                ..default()
            }
        )).id());
        return;
    };
    let window = *window;
    let Some(cursor_win) = window.cursor_position() else { return; }; // top-left origin (Bevy >= 0.11)
    let (camera, cam_transform) = match camera_q.single() {
        Ok(v) => v,
        Err(_) => return,
    };

    // canvas.size = physical size of image (tw,th)
    // canvas.window_size = current window logical size (w,h)
    let image_size = canvas.size;          // Vec2: image pixel size (physical/logical as you track it)
    let window_size = canvas.window_size;  // Vec2: window size used during resize

    // compute top-left offset where the image is blitted in the window
    let offset = (window_size - image_size) * 0.5;

    // cursor relative to top-left of image (same origin orientation as Window::cursor_position)
    let local = cursor_win - offset;

    // outside the canvas -> nothing to do
    // if local.x < 0.0 || local.y < 0.0 || local.x > image_size.x || local.y > image_size.y {
    //     return;
    // }

    // If camera has a custom viewport (or to be defensive), subtract the camera's physical viewport min.
    // Camera::physical_viewport_rect() returns a URect with top-left origin in physical pixels.
    let viewport_pos = if let Some(ur) = camera.physical_viewport_rect() {
        // URect.min is UVec2 (physical coords). Convert to f32 and subtract.
        let min = Vec2::new(ur.min.x as f32, ur.min.y as f32);
        local - min
    } else {
        // no camera viewport → local already is in full render target coords
        local
    };

    // convert to world; this uses the camera's transform and projection (accounts for rotation, scale, origin)
    match camera.viewport_to_world_2d(cam_transform, viewport_pos) {
        Ok(world_pos) => {
            // world_pos: Vec2 in your world coordinate system (x,y)
            info!("cursor world pos: {:?}", world_pos);
            cmd.entity(e).insert(
                Transform::from_translation(world_pos.extend(0.0))
            );
            // use world_pos here...
        }
        Err(err) => {
            warn!("viewport_to_world_2d failed: {:?}", err);
        }
    }

    // // mouse_pos.0 = window.cursor_position();
    // // println!("{:?}", mouse_pos.0)
    // let (camera, camera_transform) = *outer_camera_q;
    // if let Some(cursor_position) = window.cursor_position()
    //     && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    // {

    //     let p = cursor_world_pos;

    //     // let (c, t) = &mut *player;
    //     // c.look_dir = (p - t.translation().truncate()).normalize_or_zero();
    //     // info!("Mouse pos: {}", p);
    //     cmd.entity(e).insert(
    //         Transform::from_translation(p.extend(0.0))
    //     );
    // }
    outer_camera_q: Single<(&Camera, &GlobalTransform), With<HighresCamera>,>,
    world_camera_q: Single<&GlobalTransform, With<WorldCamera>,>,
    rects: Res<Rects>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *outer_camera_q;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        let p = cursor_world_pos + world_camera_q.translation().truncate();
        println!("{:?}", p);
    }
    gizmos.rect_2d(rects.red_left.center(), rects.red_left.size(), Color::Srgba(RED));
    gizmos.rect_2d(rects.white_right.center(), rects.white_right.size(), Color::Srgba(RED));
    gizmos.rect_2d(rects.faz.center(), rects.faz.size(), Color::Srgba(RED));
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
    mut fnaf_assets: ResMut<FNAFAssets>,
    mut world_camera: Query<&mut Projection, With<WorldCamera>>
) {
    let mut w = world_camera.single_mut().unwrap();
    let Projection::Orthographic(p) = &mut *w else {return;};
    p.scale = 1.0;
    // cmd.insert_resource(MousePos {0: None});
    // cmd.spawn((
    //     DespawnOnExit(STATE),
    //     Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //     Sprite {
    //         image: fnaf_assets.room.clone(),
    //         ..default()
    //     },
    // )).observe(|mut event: On<Pointer<Click>>|{info!("Click: {:?}", event)});

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
}