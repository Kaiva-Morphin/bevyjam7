use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{games::plugin::{AppState, LastState}, prelude::*};

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

fn update_mouse_pos(
    window: Single<&Window>,
    mut mouse_pos: ResMut<MousePos>,
) {
    mouse_pos.0 = window.cursor_position();
    println!("{:?}", mouse_pos.0)
}

fn tick_transition(
    mut state: ResMut<NextState<LocalState>>,
) {
    state.set(LocalState::Game);
}

fn setup(
    mut cmd: Commands,
    mut fnaf_assets: ResMut<FNAFAssets>,
) {
    cmd.insert_resource(MousePos {0: None});
    cmd.spawn((
        DespawnOnExit(STATE),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Sprite {
            image: fnaf_assets.room.clone(),
            ..default()
        },
    ));

}