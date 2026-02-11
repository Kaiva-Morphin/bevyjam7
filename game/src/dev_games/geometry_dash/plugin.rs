use avian2d::{dynamics::solver::solver_body::InertiaFlags, math::Vector};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;
pub struct GeometryDashPlugin;

const STATE: AppState = AppState::Geometry;
const NEXT_STATE: AppState = AppState::PacmanEnter;

const WIDTH : f32 = 576.0;
const HALF_HEIGHT : f32 = 250.0 / 2.0;
const SCALE : f32 = 1.0;

const DEATH_DELAY : f32 = 1.0;

const GRAVITY_SCALE : f32 = 30.;

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

#[derive(Resource)]
pub struct IsLeft {
    is: bool,
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
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick_transition.run_if(in_state(LocalState::InitialAnim)))
            .add_systems(OnEnter(LocalState::Game), begin_game)
            .add_systems(Update, (controller).run_if(in_state(LocalState::Game)))
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
    mut state: ResMut<LastState>,
) {
    state.state = STATE;
    
    cmd.spawn((
        DespawnOnExit(STATE),
        TiledMap(assets.tilemap_handle.clone()),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    cmd.insert_resource(PlayerEntity {entity: Entity::PLACEHOLDER});
    cmd.insert_resource(IsLeft {is: false});
}
fn begin_game (
    mut state: ResMut<NextState<LocalState>>
) {
    state.set(LocalState::Game);
}
   
fn tick_transition(
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
    mut player_entity: ResMut<PlayerEntity>,
) {
    let e = event.entity;
    let transform = spawnpoint_transform_q.get(e).expect("no spawnpoint").clone();

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    let collider = Collider::rectangle(16.0, 16.0);
    
    let mut caster_shape = collider.clone();
    caster_shape.set_scale(Vector::ONE * Vector::new(0.99, 0.99), 10);
    
    let layers = CollisionLayers::new(
        CollisionLayer::Default,
        [CollisionLayer::Yellow],
    );
    player_entity.entity = cmd.spawn((
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
        collider,
        RigidBody::Dynamic,
        LockedAxes::new().lock_rotation(),
        children![
            ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::X)
                .with_max_distance(8.01)
                .with_ignore_self(true)
                .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End])),
            ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::NEG_X)
                .with_max_distance(8.01)
                .with_ignore_self(true)
                .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End])),
            ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::Y)
                .with_max_distance(8.01)
                .with_ignore_self(true)
                .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End])),
            ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(8.01)
                .with_ignore_self(true)
                .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End])),
        ],
        GravityScale(GRAVITY_SCALE),
        layers,
    )).id();
}

#[derive(Component)]
struct Yellow;

#[derive(Component)]
struct White;

#[derive(Component)]
struct Aboba;

#[derive(Component)]
struct End;

#[derive(PhysicsLayer, Default)]
enum CollisionLayer {
    #[default]
    Default,
    Yellow,
    White,
    Aboba,
    End,
}

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
    if layer.name == "yellow_front" || layer.name == "yellow_back" {
        let layers = CollisionLayers::new(
        CollisionLayer::Yellow,
        [CollisionLayer::Yellow, CollisionLayer::Default],
        );
        commands.entity(collider_created.event().origin).insert((
            Yellow,
            RigidBody::Static,
            layers,
        ));
    }
    if layer.name == "white_front" || layer.name == "white_back" {
        let layers = CollisionLayers::new(
        CollisionLayer::White,
        [CollisionLayer::White, CollisionLayer::Default],
        );
        commands.entity(collider_created.event().origin).insert((
            White,
            RigidBody::Static,
            layers,
        ));
    }
    if layer.name == "white_aboba" || layer.name == "yellow_aboba" {
        let layers = CollisionLayers::new(
        CollisionLayer::Aboba,
        [CollisionLayer::Default],
        );
        commands.entity(collider_created.event().origin).insert((
            Aboba,
            RigidBody::Static,
            layers,
        ));
    }
    if layer.name == "white_end" {
        let layers = CollisionLayers::new(
        CollisionLayer::End,
        [CollisionLayer::Default],
        );
        commands.entity(collider_created.event().origin).insert((
            End,
            RigidBody::Static,
            layers,
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

const MS : f32 = 90.0;

fn controller(
    mut cmd: Commands,
    mut cube_vel_q: Query<&mut LinearVelocity, With<Cube>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    shapecast_q: Query<(&ShapeCaster, &ShapeHits)>,
    aboba_q: Query<&Aboba>,
    end_q: Query<&End>,
    mut state: ResMut<NextState<LocalState>>,
    player_entity: Res<PlayerEntity>,
    mut cube_pos_q: Query<(&mut Position, &mut CollisionLayers), With<Cube>>,
    mut is_left: ResMut<IsLeft>,
) {
    let mut on_ground = false;
    for (casters, hits) in &shapecast_q {
        let dir = casters.direction;
        for hit in hits.iter() {
            match dir {
                Dir2::NEG_Y => {
                    if hit.entity != player_entity.entity && hit.distance < 0.1 {
                        // println!("FLOOR {} {}", hit.distance, hit.entity);
                        on_ground = true;
                    }
                },
                _ => {
                    // println!("WALLS {} {}", hit.distance, hit.entity);
                    if hit.entity != player_entity.entity && hit.distance < 0.1 {
                        if let Ok(_) = aboba_q.get(hit.entity) {
                            println!("HIT ABOBA");
                            let (mut pos, mut layers) = cube_pos_q.single_mut().expect("no cube");
                            *pos = Position::from_xy(pos.x, pos.y + -1. * 6. * 16.);
                            is_left.is = !is_left.is;

                            if layers.filters == LayerMask::from([CollisionLayer::Yellow]) {
                                layers.filters = LayerMask::from([CollisionLayer::White]);
                                cmd.entity(player_entity.entity).despawn_children();
                                let collider = Collider::rectangle(16.0, 16.0);
    
                                let mut caster_shape = collider.clone();
                                caster_shape.set_scale(Vector::ONE * Vector::new(0.99, 0.99), 10);
                                cmd.entity(player_entity.entity).insert(children![
                                    ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::X)
                                        .with_max_distance(8.01)
                                        .with_ignore_self(true)
                                        .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::White, CollisionLayer::Aboba, CollisionLayer::End])),
                                    ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::NEG_X)
                                        .with_max_distance(8.01)
                                        .with_ignore_self(true)
                                        .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::White, CollisionLayer::Aboba, CollisionLayer::End])),
                                    ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::Y)
                                        .with_max_distance(8.01)
                                        .with_ignore_self(true)
                                        .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::White, CollisionLayer::Aboba, CollisionLayer::End])),
                                    ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::NEG_Y)
                                        .with_max_distance(8.01)
                                        .with_ignore_self(true)
                                        .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::White, CollisionLayer::Aboba, CollisionLayer::End])),
                                ],);
                            } else {
                                layers.filters = LayerMask::from([CollisionLayer::Yellow]);
                                cmd.entity(player_entity.entity).despawn_children();
                                let collider = Collider::rectangle(16.0, 16.0);
    
                                let mut caster_shape = collider.clone();
                                caster_shape.set_scale(Vector::ONE * Vector::new(0.99, 0.99), 10);
                                cmd.entity(player_entity.entity).insert(children![
                                    ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::X)
                                        .with_max_distance(8.01)
                                        .with_ignore_self(true)
                                        .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End])),
                                    ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::NEG_X)
                                        .with_max_distance(8.01)
                                        .with_ignore_self(true)
                                        .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End])),
                                    ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::Y)
                                        .with_max_distance(8.01)
                                        .with_ignore_self(true)
                                        .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End])),
                                    ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::NEG_Y)
                                        .with_max_distance(8.01)
                                        .with_ignore_self(true)
                                        .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End])),
                                ],);
                            }
                            return;
                        } else if let Ok(_) = end_q.get(hit.entity) {
                            println!("HIT END");
                            return;
                        } else {
                            state.set(LocalState::Defeat);
                        }
                    }
                }
            }
        }
    }
    let mut vel = cube_vel_q.single_mut().expect("no cube(");
    if !is_left.is {
        vel.x = MS;
    } else {
        vel.x = -MS;
    }
    // println!("{} {}", keyboard_input.pressed(KeyCode::Space), on_ground);
    if keyboard_input.pressed(KeyCode::Space) && on_ground {
        vel.y = 160.;
    }
}

fn tick_defeat(
    mut t: Local<f32>,
    time: Res<Time>,
    mut state: ResMut<NextState<AppState>>,
){
    let dt = time.delta_secs().min(MAX_DT);
    *t += dt;
    if *t >= DEATH_DELAY {
        state.set(AppState::Defeat);
    }
}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
) {
    cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
    cmd.remove_resource::<PlayerEntity>();
    cmd.remove_resource::<IsLeft>();
}