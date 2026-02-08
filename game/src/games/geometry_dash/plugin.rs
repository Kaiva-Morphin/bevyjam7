use avian2d::dynamics::solver::solver_body::InertiaFlags;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{games::plugin::AppState, prelude::*};
pub struct GeometryDashPlugin;

const STATE: AppState = AppState::Geometry;
const NEXT_STATE: AppState = AppState::PacmanEnter;

const WIDTH : f32 = 576.0;
const HALF_HEIGHT : f32 = 250.0 / 2.0;
const SCALE : f32 = 1.0;

const DEATH_DELAY : f32 = 1.0;

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

#[derive(Resource)]
pub struct PlayerEntity {
    entity: Entity,
}

impl Plugin for GeometryDashPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CameraCenter>()
            .register_type::<SpawnPoint>()
            .add_sub_state::<LocalState>()
            .add_observer(spawnpoint_handler)
            .add_observer(camera_handler)
            .add_observer(on_collider_spawned)
            .insert_resource(PlayerEntity {entity: Entity::PLACEHOLDER})
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick_transition.run_if(in_state(LocalState::InitialAnim)))
            // .add_systems(OnEnter(LocalState::Game), begin_game)
            .add_systems(Update, (controller, print_started_collisions).run_if(in_state(LocalState::Game)))
            .add_systems(Update, tick_defeat.run_if(in_state(LocalState::Defeat)))
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
    mut plyer_entity: ResMut<PlayerEntity>,
) {
    let e = event.entity;
    let transform = spawnpoint_transform_q.get(e).expect("no spawnpoint").clone();

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player = cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Pacman"),
        LinearVelocity(Vec2::ZERO),
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
        Collider::rectangle(16.0, 16.0),
        CollisionEventsEnabled,
        RigidBody::Dynamic,
        LockedAxes::new().lock_rotation(),
        children![(
            RayCaster::new(Vec2::ZERO, Dir2::NEG_Y),
        )],
        GravityScale(5.),
    )).id();
    plyer_entity.entity = player;
}

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Wall;

fn on_collider_spawned(
    collider_created: On<TiledEvent<ColliderCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
    state: Res<State<AppState>>,
) {
    if state.get() != &STATE {
        return;
    }
    let Some(layer) = collider_created.event().get_layer(&assets) else {return;};
    if layer.name == "yellow_floor" {
        commands.entity(collider_created.event().origin).insert((
            Floor,
            RigidBody::Static,
        ));
    }
    if layer.name == "yellow_walls" {
        commands.entity(collider_created.event().origin).insert((
            Wall,
            RigidBody::Static,
        ));
    }
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

const MS : f32 = 30.0;
const G : f32 = 9.81 * 3.;

fn controller(
    mut cmd: Commands,
    mut cube_vel_q: Query<&mut LinearVelocity, With<Cube>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    raycast_q: Query<(&RayCaster, &RayHits)>,
    floor_q: Query<&Floor>,
    plyer_entity: Res<PlayerEntity>,
) {
    let mut on_ground = false;
    for (ray, hits) in &raycast_q {
        for hit in hits.iter_sorted() {
            if let Ok(_) = floor_q.get(hit.entity) {
                // println!("{}", hit.distance);
                if hit.distance < 8.01 {
                    on_ground = true;
                    break;
                }
            }
        }
    }
    let mut vel = cube_vel_q.single_mut().expect("no cube(");
    vel.x = MS;
    // println!("{} {}", keyboard_input.pressed(KeyCode::Space), on_ground);
    if keyboard_input.pressed(KeyCode::Space) && on_ground {
        vel.y = 60.;
    }
}

fn tick_defeat(
    mut t: Local<f32>,
    time: Res<Time>,
    mut state: ResMut<NextState<AppState>>,
    mut cube_transform_q: Query<&mut Transform, With<Cube>>,
    spawnpoint_transform_q: Query<&Transform, With<SpawnPoint>>,
){
    let dt = time.delta_secs().min(MAX_DT);
    *t += dt;
    if *t >= DEATH_DELAY {
        let mut cube_t = cube_transform_q.single_mut().expect("no cube");
        let spawnpoint_t = spawnpoint_transform_q.single().expect("no spawnpoint").clone();
        *cube_t = spawnpoint_t;
    }
}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
) {
    cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
    cmd.remove_resource::<PlayerEntity>();
}

fn print_started_collisions(mut collision_reader: MessageReader<CollisionStart>) {
    for event in collision_reader.read() {
        println!("{} and {} started colliding", event.collider1, event.collider2);
    }
}