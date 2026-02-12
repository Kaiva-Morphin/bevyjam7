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