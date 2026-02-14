use bevy::{audio::Volume, color::palettes::css::{RED, WHITE}};
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;

use crate::prelude::*;

pub struct FNAFPlugin;

const STATE: AppState = AppState::Fnaf;
const NEXT_STATE: AppState = AppState::FakeEnd;

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
    #[asset(path = "sounds/fnaf/ambience.mp3")]
    ambience: Handle<AudioSource>,
    #[asset(path = "sounds/fnaf/open_door.mp3")]
    open_door: Handle<AudioSource>,
    #[asset(path = "sounds/fnaf/close_door.mp3")]
    close_door: Handle<AudioSource>,
    #[asset(path = "sounds/fnaf/light.mp3")]
    light: Handle<AudioSource>,
    #[asset(path = "sounds/fnaf/lobster.mp3")]
    lobster_audio: Handle<AudioSource>,
    #[asset(path = "sounds/fnaf/faz.mp3")]
    faz: Handle<AudioSource>,
    #[asset(path = "sounds/fnaf/yay.mp3")]
    yay: Handle<AudioSource>,
    #[asset(path = "sounds/fnaf/mem.mp3")]
    mem: Handle<AudioSource>,
    #[asset(path = "sounds/fnaf/ur.mp3")]
    ur: Handle<AudioSource>,

    #[asset(path = "images/fnaf/room.png")]
    room: Handle<Image>,
    #[asset(path = "images/fnaf/white.png")]
    white_button: Handle<Image>,
    #[asset(path = "images/fnaf/red.png")]
    red_button: Handle<Image>,
    #[asset(path = "images/fnaf/door.png")]
    door: Handle<Image>,
    #[asset(path = "images/fnaf/light_l.png")]
    light_l: Handle<Image>,
    #[asset(path = "images/fnaf/light_R.png")]
    light_r: Handle<Image>,
    #[asset(path = "images/fnaf/fred_l.png")]
    fred_l: Handle<Image>,
    #[asset(path = "images/fnaf/fred_r.png")]
    fred_r: Handle<Image>,
    #[asset(path = "images/fnaf/lobster.jpg")]
    lobster_pic: Handle<Image>,
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
            .add_systems(Update, (
                update_mouse_pos, update_text, handle_faz_time, play_mem, handle_faz,
                (handle_rects, handle_game_logic).chain()).run_if(in_state(LocalState::Game)))
            .add_systems(Update, defeat.run_if(in_state(LocalState::Defeat)))
            .add_systems(Update, win.run_if(in_state(LocalState::Win)))
            .add_systems(OnExit(STATE), cleanup)
            ;
    }
}

#[derive(Resource)]
pub struct MousePos(pub Option<Vec2>);

#[derive(Component)]
pub struct DbgSprite;

#[derive(Component)]
pub struct BatteryText;

#[derive(Component)]
pub struct TimeText;

#[derive(Component)]
pub struct TimeText1;

#[derive(Resource)]
pub struct Rects {
    pub red_right: Rect,
    pub red_left: Rect,
    pub white_right: Rect,
    pub white_left: Rect,
    pub faz: Rect,
}

#[derive(Resource, Default)]
pub struct EnvironmentData {
    left_door_open: bool,
    right_door_open: bool,
    left_light_on: bool,
    right_light_on: bool,
}

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
            pos.0 = Some(world_pos)
        }
        Err(err) => {
            warn!("viewport_to_world_2d failed: {:?}", err);
        }
    }
}

#[derive(Component)]
pub struct LightAudio;

fn handle_rects(
    mut cmd: Commands,
    rects: Res<Rects>,
    mouse_pos: Res<MousePos>,
    mut gizmos: Gizmos,
    mouse_input: Res<ButtonInput<MouseButton>>,
    fnaf_assets: Res<FNAFAssets>,
    mut env_data: ResMut<EnvironmentData>,
    mut env: Query<(&mut Visibility, &Environment)>,
    light_audio: Query<Entity, With<LightAudio>>,
    bear_data: Res<BearData>,
) {
    gizmos.rect_2d(rects.faz.center(), rects.faz.size(), Color::Srgba(RED));
    gizmos.rect_2d(rects.red_left.center(), rects.red_left.size(), Color::Srgba(RED));
    if let Some(pos) = mouse_pos.0 {
        if mouse_input.just_pressed(MouseButton::Left) {
            if rects.faz.contains(pos) {
                cmd.spawn((
                    DespawnOnExit(STATE),
                    AudioPlayer(fnaf_assets.faz.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Once,
                        ..default()
                    },
                ));
            } else if rects.red_left.contains(pos) {
                if env_data.left_door_open {
                    cmd.spawn((
                        DespawnOnExit(STATE),
                        AudioPlayer(fnaf_assets.open_door.clone()),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Once,
                            ..default()
                        },
                    ));
                    for (mut visibility, env) in env.iter_mut() {
                        match *env {
                            Environment::LDoor => {
                                *visibility = Visibility::Visible
                            },
                            Environment::LRedlight => {
                                *visibility = Visibility::Visible
                            },
                            _ => {}
                        }
                    }
                } else {
                    cmd.spawn((
                        DespawnOnExit(STATE),
                        AudioPlayer(fnaf_assets.close_door.clone()),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Once,
                            ..default()
                        },
                    ));
                    for (mut visibility, env) in env.iter_mut() {
                        match *env {
                            Environment::LDoor => {
                                *visibility = Visibility::Hidden
                            },
                            Environment::LRedlight => {
                                *visibility = Visibility::Hidden
                            },
                            _ => {}
                        }
                    }
                }
                env_data.left_door_open = !env_data.left_door_open;
            } else if rects.red_right.contains(pos) {
                if env_data.right_door_open {
                    cmd.spawn((
                        DespawnOnExit(STATE),
                        AudioPlayer(fnaf_assets.open_door.clone()),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Once,
                            ..default()
                        },
                    ));
                    for (mut visibility, env) in env.iter_mut() {
                        match *env {
                            Environment::RDoor => {
                                *visibility = Visibility::Visible
                            },
                            Environment::RRedlight => {
                                *visibility = Visibility::Visible
                            },
                            _ => {}
                        }
                    }
                } else {
                    cmd.spawn((
                        DespawnOnExit(STATE),
                        AudioPlayer(fnaf_assets.close_door.clone()),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Once,
                            ..default()
                        },
                    ));
                    for (mut visibility, env) in env.iter_mut() {
                        match *env {
                            Environment::RDoor => {
                                *visibility = Visibility::Hidden
                            },
                            Environment::RRedlight => {
                                *visibility = Visibility::Hidden
                            },
                            _ => {}
                        }
                    }
                }
                env_data.right_door_open = !env_data.right_door_open;
            }
        }

        if rects.white_left.contains(pos) {
            if mouse_input.just_pressed(MouseButton::Left) {
                env_data.left_light_on = true;
                cmd.spawn((
                    DespawnOnExit(STATE),
                    LightAudio,
                    AudioPlayer(fnaf_assets.light.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Once,
                        volume: Volume::Linear(0.5),
                        ..default()
                    },
                ));
                for (mut visibility, env) in env.iter_mut() {
                    match *env {
                        Environment::LWhitelight => {
                            *visibility = Visibility::Visible
                        }
                        Environment::LLight => {
                            if !bear_data.bear_here || bear_data.right {
                                *visibility = Visibility::Visible
                            }
                        }
                        Environment::LBear => {
                            if bear_data.bear_here && !bear_data.right {
                                *visibility = Visibility::Visible;
                                cmd.spawn((
                                    DespawnOnExit(STATE),
                                    AudioPlayer(fnaf_assets.ur.clone()),
                                    PlaybackSettings {
                                        mode: bevy::audio::PlaybackMode::Once,
                                        ..default()
                                    },
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
            if mouse_input.just_released(MouseButton::Left) {
                env_data.left_light_on = false;
                for audio in light_audio {
                    cmd.entity(audio).despawn();
                }
                for (mut visibility, env) in env.iter_mut() {
                    match *env {
                        Environment::LWhitelight => {
                            *visibility = Visibility::Hidden
                        }
                        Environment::LLight => {
                            *visibility = Visibility::Hidden
                        }
                        Environment::LBear => {
                            *visibility = Visibility::Hidden
                        }
                        _ => {}
                    }
                }
            }
        } else if rects.white_right.contains(pos) {
            if mouse_input.just_pressed(MouseButton::Left) {
                env_data.right_light_on = true;
                cmd.spawn((
                    DespawnOnExit(STATE),
                    LightAudio,
                    AudioPlayer(fnaf_assets.light.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Once,
                        volume: Volume::Linear(0.5),
                        ..default()
                    },
                ));
                for (mut visibility, env) in env.iter_mut() {
                    match *env {
                        Environment::RWhitelight => {
                            *visibility = Visibility::Visible
                        }
                        Environment::RLight => {
                            if !bear_data.bear_here || !bear_data.right {
                                *visibility = Visibility::Visible
                            }
                        }
                        Environment::RBear => {
                            if bear_data.bear_here && bear_data.right {
                                *visibility = Visibility::Visible;
                                cmd.spawn((
                                    DespawnOnExit(STATE),
                                    AudioPlayer(fnaf_assets.ur.clone()),
                                    PlaybackSettings {
                                        mode: bevy::audio::PlaybackMode::Once,
                                        ..default()
                                    },
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
            if mouse_input.just_released(MouseButton::Left) {
                env_data.right_light_on = false;
                for audio in light_audio {
                    cmd.entity(audio).despawn();
                }
                for (mut visibility, env) in env.iter_mut() {
                    match *env {
                        Environment::RWhitelight => {
                            *visibility = Visibility::Hidden
                        }
                        Environment::RLight => {
                            *visibility = Visibility::Hidden
                        }
                        Environment::RBear => {
                            *visibility = Visibility::Hidden
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct Battery {
    pub charge: f32
}

#[derive(Resource)]
pub struct FazTime{
    pub start_time: f32,
    pub time_to_show: usize,
}

fn handle_game_logic(
    env_data: Res<EnvironmentData>,
    mut battery: ResMut<Battery>,
    time: Res<Time>,
    mut state: ResMut<NextState<LocalState>>,
) {
    let dt = time.delta_secs();
    const DOOR_DISCHARGE: f32 = 1.25;
    const WINDOW_DISCHARGE: f32 = 0.4;
    let mut discharge = 0.;
    if !env_data.left_door_open {
        discharge += DOOR_DISCHARGE * dt;
    }
    if !env_data.right_door_open {
        discharge += DOOR_DISCHARGE * dt;
    }
    if env_data.left_light_on {
        discharge += WINDOW_DISCHARGE * dt;
    }
    if env_data.right_light_on {
        discharge += WINDOW_DISCHARGE * dt;
    }
    battery.charge -= discharge;
    if battery.charge <= 0. {
        state.set(LocalState::Defeat); //todo: add winscreen
    }
}

fn update_text(
    battery: Res<Battery>,
    faz_time: Res<FazTime>,
    mut battery_text: Query<&mut TextSpan, (With<BatteryText>, Without<TimeText>, Without<TimeText1>)>,
    mut time_text: Query<&mut TextSpan, (With<TimeText>, Without<BatteryText>, Without<TimeText1>)>,
    mut time_text1: Query<&mut TextSpan, (With<TimeText1>, Without<BatteryText>, Without<TimeText>)>,
) {
    let charge = battery.charge;
    for mut text in battery_text.iter_mut() {
        **text = format!("{charge:.0}");
    }
    let t = faz_time.time_to_show;
    for mut text in time_text.iter_mut() {
        **text = format!("{t:.0}");
    }
    let tt;
    if t == 12 {
        tt = " PM"
    } else {
        tt = " AM"
    }
    for mut text in time_text1.iter_mut() {
        **text = format!("{tt}");
    }
}

fn handle_faz_time(
    mut faz_time: ResMut<FazTime>,
    time: Res<Time>,
    mut state: ResMut<NextState<LocalState>>,
) {
    if faz_time.start_time == 0. {
        faz_time.start_time = time.elapsed_secs()
    } else {
        let elapsed = time.elapsed_secs() - faz_time.start_time;
        if elapsed < 20. * 1. {
            faz_time.time_to_show = 12;
        } else if elapsed < 20. * 2. {
            faz_time.time_to_show = 1;
        } else if elapsed < 20. * 3. {
            faz_time.time_to_show = 2;
        } else if elapsed < 20. * 4. {
            faz_time.time_to_show = 3;
        } else if elapsed < 20. * 5. {
            faz_time.time_to_show = 4;
        } else if elapsed < 20. * 6. {
            faz_time.time_to_show = 5;
        } else if elapsed < 20. * 7. {
            faz_time.time_to_show = 6;
            state.set(LocalState::Win);
        }
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
    LLight,
    RLight,
    Faz,
    LBear,
    RBear,
}

fn setup(
    mut cmd: Commands,
    fnaf_assets: Res<FNAFAssets>,
    asset_server: Res<AssetServer>,
    mut proj: Query<&mut Projection, With<WorldCamera>>,
    mut state: ResMut<LastState>,
) {
    state.state = STATE;
    cmd.insert_resource(MousePos(None));
    cmd.insert_resource(EnvironmentData {
        left_door_open: true,
        right_door_open: true,
        left_light_on: false,
        right_light_on: false,
    });
    cmd.insert_resource(FazTime {start_time: 0., time_to_show: 12});
    cmd.insert_resource(Battery {charge: 100.});
    cmd.insert_resource(MemTimer {timer: 0., disable: false});
    cmd.insert_resource(BearData::default());
    cmd.insert_resource(WinscreenTimer::default());
    cmd.insert_resource(LobsterTimer::default());
    cmd.spawn((
        DespawnOnExit(STATE),
        AudioPlayer(fnaf_assets.ambience.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
        },
    ));
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
        Transform::from_xyz(0., 0., 10.),
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.door.clone(),
            flip_x: true,
            ..default()
        },
        Environment::RDoor,
        Transform::from_xyz(0., 0., 10.),
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.white_button.clone(),
            flip_x: true,
            ..default()
        },
        Environment::LWhitelight,
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.white_button.clone(),
            ..default()
        },
        Environment::RWhitelight,
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.red_button.clone(),
            flip_x: true,
            ..default()
        },
        Environment::LRedlight,
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.red_button.clone(),
            ..default()
        },
        Environment::RRedlight,
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.light_l.clone(),
            ..default()
        },
        Environment::LLight,
        Transform::from_xyz(0., 0., 1.),
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.light_r.clone(),
            ..default()
        },
        Environment::RLight,
        Transform::from_xyz(0., 0., 1.),
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.fred_l.clone(),
            ..default()
        },
        Environment::LBear,
        Transform::from_xyz(0., 0., 2.),
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.fred_r.clone(),
            ..default()
        },
        Environment::RBear,
        Transform::from_xyz(0., 0., 2.),
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.lobster_pic.clone(),
            flip_x: true,
            ..default()
        },
        Lobster,
        Transform::from_xyz(0., 0., 100.),
        Visibility::Hidden,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Sprite {
            image: fnaf_assets.lobster_pic.clone(), // todo: change
            flip_x: true,
            ..default()
        },
        Winscreen,
        Visibility::Hidden,
    ));
    let font = asset_server.load("fonts/kaivs_minegram_v1.ttf");
    cmd.spawn((
        DespawnOnExit(STATE),
        Text::new("Battery: "),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(50.0),
            ..default()
        },
        TextFont {
            font: font.clone(),
            font_size: 33.0,
            ..default()
        },
    )).with_child((
        TextSpan::default(),
        TextFont {
            font: font.clone(),
            font_size: 33.0,
            ..Default::default()
        },
        TextColor(Color::Srgba(WHITE)),
        BatteryText,
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Percent(50.),
            ..default()
        },
        TextFont {
            font: font.clone(),
            font_size: 33.0,
            ..default()
        },
    )).with_child((
        TextSpan::default(),
        TextFont {
            font: font.clone(),
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::Srgba(WHITE)),
        TimeText,
    )).with_child((
        TextSpan::default(),
        TextFont {
            font: font.clone(),
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::Srgba(WHITE)),
        TimeText1,
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
    cmd.remove_resource::<EnvironmentData>();
    cmd.remove_resource::<Battery>();
    cmd.remove_resource::<FazTime>();
    cmd.remove_resource::<MemTimer>();
    cmd.remove_resource::<BearData>();
    cmd.remove_resource::<LobsterTimer>();
    cmd.remove_resource::<WinscreenTimer>();
}

#[derive(Component)]
pub struct Lobster;

#[derive(Resource, Default)]
pub struct LobsterTimer(f32);

fn defeat(
    mut cmd: Commands,
    fnaf_assets: Res<FNAFAssets>,
    mut lobster: Query<&mut Visibility, With<Lobster>>,
    time: Res<Time>,
    mut screenshot: ResMut<LastScreenshot>,
    mut lobster_timer: ResMut<LobsterTimer>,
) {
    let mut visibility = lobster.single_mut().unwrap();
    if lobster_timer.0 == 0. {
        *visibility = Visibility::Visible;
        cmd.spawn((
            DespawnOnExit(STATE),
            AudioPlayer(fnaf_assets.lobster_audio.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Once,
                ..default()
            },
        ));
    } else {
        if lobster_timer.0 > 1. {
            if screenshot.awaiting == false {
                cmd.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                    .observe(await_screenshot_and_translate(AppState::Defeat));
                screenshot.awaiting = true;
            }
        }
    }
    lobster_timer.0 += time.delta_secs();
}

#[derive(Component)]
pub struct Winscreen;

#[derive(Resource, Default)]
pub struct WinscreenTimer(f32);
fn win(
    mut cmd: Commands,
    mut winscreen: Query<&mut Visibility, With<Winscreen>>,
    fnaf_assets: Res<FNAFAssets>,
    time: Res<Time>,
    mut state: ResMut<NextState<AppState>>,
    mut winscreen_timer: ResMut<WinscreenTimer>,
) {
    let mut visibility = winscreen.single_mut().unwrap();
    if winscreen_timer.0 == 0. {
        *visibility = Visibility::Visible;
        cmd.spawn((
            DespawnOnExit(STATE),
            AudioPlayer(fnaf_assets.yay.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Once,
                volume: Volume::Linear(1.),
                ..default()
            },
        ));
    } else {
        if winscreen_timer.0 > 2. {
            state.set(NEXT_STATE);
        }
    }
    winscreen_timer.0 += time.delta_secs();
}

#[derive(Resource)]
pub struct MemTimer {
    pub timer: f32,
    pub disable: bool,
}

fn play_mem(
    mut cmd: Commands,
    mut mem_timer: ResMut<MemTimer>,
    time: Res<Time>,
    fnaf_assets: Res<FNAFAssets>,
) {
    mem_timer.timer += time.delta_secs();
    if mem_timer.timer < 5. && !mem_timer.disable {
        cmd.spawn((
            DespawnOnExit(STATE),
            AudioPlayer(fnaf_assets.mem.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Once,
                volume: Volume::Linear(1.),
                ..default()
            },
        ));
        mem_timer.disable = true
    }
}

#[derive(Resource, Default)]
pub struct BearData {
    pub initial_delay: f32,
    pub bear_until_comes: f32,
    pub bear_until_leaves: f32,
    pub bear_until_kills: f32,
    pub bear_here: bool,
    pub right: bool,
}

fn handle_faz(
    time: Res<Time>,
    env_data: Res<EnvironmentData>,
    mut bear_data: ResMut<BearData>,
    mut state: ResMut<NextState<LocalState>>,
) {
    let mut rng = rand::rng();
    let delta_secs = time.delta_secs();
    if bear_data.initial_delay > 2. { // todo: set to 20.
        if bear_data.bear_here {
            if bear_data.bear_until_leaves <= 0. {
                println!("BEAR LEFT");
                bear_data.bear_here = false;
                bear_data.bear_until_kills = 0.;
                bear_data.bear_until_comes = rng.random_range(20.0..30.0);
                println!("BEAR COMES IN {}", bear_data.bear_until_comes);
            } else {
                bear_data.bear_until_leaves -= delta_secs;
            }
            if (env_data.left_door_open && !bear_data.right) || (env_data.right_door_open && bear_data.right) {
                bear_data.bear_until_kills += delta_secs;
            } else {
                bear_data.bear_until_kills = 0.;
            }
            if bear_data.bear_until_kills >= 10. {
                state.set(LocalState::Defeat);
            }
        } else {
            if bear_data.bear_until_comes <= 0. {
                println!("BEAR HERE");
                bear_data.bear_here = true;
                bear_data.right = rng.random_bool(0.5);
                bear_data.bear_until_leaves = rng.random_range(10.0..20.0);
                println!("BEAR LEAVES IN {}", bear_data.bear_until_leaves);
            } else {
                bear_data.bear_until_comes -= delta_secs;
            }
        }
    } else {
        bear_data.initial_delay += delta_secs;
    }
}