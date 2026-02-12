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
    #[asset(path = "images/fnaf/button.png")]
    button: Handle<Image>,
    #[asset(path = "images/fnaf/button1.png")]
    button1: Handle<Image>,
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
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick_transition.run_if(in_state(LocalState::InitialAnim)))
            // .add_systems(OnEnter(LocalState::Game), begin_game)
            .add_systems(Update, (update_mouse_pos).run_if(in_state(LocalState::Game)))
            // .add_systems(Update, tick_defeat.run_if(in_state(LocalState::Defeat)))
            // .add_systems(Update, tick_win.run_if(in_state(LocalState::Win)))
            // .add_systems(OnExit(STATE), cleanup)
            ;
    }
}

#[derive(Resource)]
pub struct MousePos(pub Option<Vec2>);

#[derive(Component)]
pub struct DbgSprite;

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
            cmd.entity(e).insert(
                Transform::from_translation(world_pos.extend(0.0))
            );
            // use world_pos here...
        }
        Err(err) => {
            warn!("viewport_to_world_2d failed: {:?}", err);
        }
    }
}

fn tick_transition(
    mut state: ResMut<NextState<LocalState>>,
) {
    state.set(LocalState::Game);
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

}