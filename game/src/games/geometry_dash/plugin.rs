use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{games::plugin::AppState, prelude::*};
pub struct GeometryDashPlugin;

const STATE: AppState = AppState::Geometry;
const NEXT_STATE: AppState = AppState::PacmanEnter;

const WIDTH : f32 = 576.0;
const HALF_HEIGHT : f32 = 250.0 / 2.0;
const SCALE : f32 = 1.0;

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

#[derive(Component)]
pub struct Cube;

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct SpawnPoint;

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct CameraCenter;

impl Plugin for GeometryDashPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CameraCenter>()
            .register_type::<SpawnPoint>()
            .add_sub_state::<LocalState>()
            .add_observer(spawnpoint_handler)
            .add_observer(camera_handler)
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick_transition.run_if(in_state(LocalState::InitialAnim)))
            // .add_systems(OnEnter(LocalState::Game), begin_game)
            // .add_systems(Update, tick_game.run_if(in_state(LocalState::Game)))
            // .add_systems(Update, tick_defat.run_if(in_state(LocalState::Defeat)))
            // .add_systems(Update, tick_win.run_if(in_state(LocalState::Win)))
            .add_systems(OnExit(STATE), cleanup)
            ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct GeometryDashAssets {
    #[asset(path = "images/pacman.png")]
    cube: Handle<Image>,
    #[asset(path = "maps/GD/pacman.tmx")]
    tilemap_handle: Handle<TiledMapAsset>,
}

fn setup(
    mut cmd: Commands,
    assets: Res<GeometryDashAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    

    
    cmd.spawn((
        DespawnOnExit(STATE),
        TiledMap(assets.tilemap_handle.clone()),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}
fn begin_game (
    mut cmd: Commands,
    // q: Query<Entity, With<Pacman>>
) {
    // cmd.entity(q.iter().next().expect("No pacman!")).insert(GravityScale(GRAVITY_AFFECT));
}
   
fn tick_transition(
    t: Res<Time>,
    mut state: ResMut<NextState<LocalState>>
){
    state.set(LocalState::Game);
}

fn spawnpoint_handler(
    event: On<Add, SpawnPoint>,
    mut cmd: Commands,
    spawnpoint_transform_q: Query<&Transform, With<SpawnPoint>>,
    assets: Res<GeometryDashAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let e = event.entity;
    let transform = spawnpoint_transform_q.get(e).expect("no player").clone();

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Pacman"),
        LinearVelocity(Vec2::X),
        transform,
        Sprite {
            image: assets.cube.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Cube,
        Collider::circle(8.0),
        CollisionEventsEnabled,
        RigidBody::Dynamic,
        GravityScale(0.0),
    ));
}

fn camera_handler(
    event: On<Add, CameraCenter>,
    center_transform_q: Query<&Transform, (With<CameraCenter>, Without<WorldCamera>)>,
    mut camera_q: Query<&mut Transform, (With<WorldCamera>, Without<CameraCenter>)>,
) {
    let e = event.entity;
    let center_transform = center_transform_q.get(e).expect("no center");
    let mut camera_t = camera_q.single_mut().expect("no camera");
    camera_t.translation = center_transform.translation;

}

fn jump(

) {

}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
) {
    cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
}