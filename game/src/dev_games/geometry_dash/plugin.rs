use std::time::Duration;

use avian2d::{dynamics::solver::solver_body::InertiaFlags, math::{PI, Vector}};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;
pub struct GeometryDashPlugin;

const STATE: AppState = AppState::Geometry;
const NEXT_STATE: AppState = AppState::Platformer;

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
            .add_systems(Update, (fix_casters, controller).chain().run_if(in_state(LocalState::Game)))
            .add_systems(Update, tick_defeat.run_if(in_state(LocalState::Defeat)))
            .add_systems(Update, tick_win.run_if(in_state(LocalState::Win)))
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
    mut state: ResMut<LastState>,
) {
    state.state = STATE;
    cmd.spawn((
        Name::new("GD map"),
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

#[derive(Component, Debug)]
pub enum CastDir {
    X,
    Y,
    NEGX,
    NEGY,
}

fn spawnpoint_handler(
    event: On<Add, SpawnPoint>,
    mut cmd: Commands,
    spawnpoint_transform_q: Query<&Transform, With<SpawnPoint>>,
    assets: Res<GeometryDashAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_entity: ResMut<PlayerEntity>,
    mut proj: Query<&mut Projection, With<WorldCamera>>
) {
    match &mut *proj.single_mut().expect("nocam") {
        Projection::Orthographic(proj) => {
            proj.scale = 2.;
        },
        _ => {}
    }
    proj.single_mut().expect("no cam");
    let e = event.entity;
    let transform = spawnpoint_transform_q.get(e).expect("no spawnpoint").clone();

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    let collider = Collider::rectangle(16.0, 16.0);
    
    let mut caster_shape = collider.clone();
    caster_shape.set_scale(Vector::ONE * Vector::new(0.95, 0.95), 10);
    
    let layers = CollisionLayers::new(
        CollisionLayer::Default,
        [CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End],
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
        children![
            (
                ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::X)
                    .with_max_distance(10.)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End]))
                    .with_max_hits(1),
                CastDir::X,
            ),
            (
                ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::NEG_X)
                    .with_max_distance(10.)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End]))
                    .with_max_hits(1),
                CastDir::NEGX,
            ),
            (
                ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::Y)
                    .with_max_distance(10.)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End]))
                    .with_max_hits(1),
                CastDir::Y,
            ),
            (
                ShapeCaster::new(caster_shape.clone(), Vector::ZERO, 0.0, Dir2::NEG_Y)
                    .with_max_distance(10.)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End]))
                    .with_max_hits(1),
                CastDir::NEGY,
            ),
        ],
        GravityScale(0.),
        layers,
        CollisionEventsEnabled,
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
    if layer.name == "white_aboba" {
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
    if layer.name == "yellow_aboba" {
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

const MS : f32 = 0.0;

fn controller(
    mut cube_vel_q: Query<&mut LinearVelocity, With<Cube>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    shapecast_q: Query<(&mut ShapeCaster, &ShapeHits, &CastDir)>,
    end_q: Query<&End>,
    mut state: ResMut<NextState<LocalState>>,
    player_entity: Res<PlayerEntity>,
    mut cube_pos_q: Query<(&mut Position, &mut CollisionLayers), With<Cube>>,
    mut is_left: ResMut<IsLeft>,
    time: Res<Time>,
    mut t: Local<Duration>,
    mut inair: Local<bool>,
    mut just_jumped: Local<bool>,
    mut cube_transform_q: Query<&mut Transform, With<Cube>>,
    aboba: Query<&Aboba>,
    mut collision_reader: MessageReader<CollisionStart>
) {
    let mut on_ground = false;
    let mut hit_aboba = false;
    for (_casters, hits, castdir) in shapecast_q.iter() {
        for hit in hits.iter() {
            match castdir {
                &CastDir::NEGY => {
                    if hit.entity != player_entity.entity && hit.distance < 0.5 {
                        // println!("FLOOR {} {} {:?}", hit.distance, hit.entity, castdir);
                        on_ground = true;
                        if *inair {
                            println!("{:?}", (time.elapsed() - *t).as_millis());
                            *inair = false;
                        }
                        let mut cube_t = cube_transform_q.single_mut().expect("no cube(");
                        let rotation_angle = cube_t.rotation.to_euler(EulerRot::XYZ).2;
                        let rot = (rotation_angle / PI * 2.0).round() * PI / 2.0;
                        cube_t.rotation = Quat::from_rotation_z(rot);
                    }
                },
                _ => {
                    // println!("WALLS {} {} {:?}", hit.distance, hit.entity, castdir);
                    if hit.entity != player_entity.entity && hit.distance <= 0.4 && hit.distance > 0.1 {
                        state.set(LocalState::Defeat);
                    }
                }
            }
        }
    }
    for event in collision_reader.read() {
        println!("{} and {} started colliding", event.collider1, event.collider2);
        if let Ok(_) = aboba.get(event.collider2) {
            hit_aboba = true;
            break;
        } else if let Ok(_) = end_q.get(event.collider2) {
            state.set(LocalState::Win);
            return;
        }
    }
    if hit_aboba {
        println!("HIT ABOBA");
        let (mut pos, mut layers) = cube_pos_q.single_mut().expect("no cube");
        let mut to_white = true;
        is_left.is = !is_left.is;
        *pos = Position::from_xy(pos.x, pos.y + -1. * 6. * 16.);
        if layers.filters == LayerMask::from([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End]) {
            layers.filters = LayerMask::from([CollisionLayer::White, CollisionLayer::Aboba, CollisionLayer::End]);
        } else {
            layers.filters = LayerMask::from([CollisionLayer::Yellow, CollisionLayer::Aboba, CollisionLayer::End]);
            to_white = false;
            
        }
        for (mut caster, _hits, _) in shapecast_q {
            if to_white {
                *caster = caster.clone().with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::White, CollisionLayer::Aboba, CollisionLayer::End]));
            } else {
                *caster = caster.clone().with_query_filter(SpatialQueryFilter::from_mask([CollisionLayer::Yellow, CollisionLayer::Aboba]));
            }
        }
    }

    let mut vel = cube_vel_q.single_mut().expect("no cube(");
    if !is_left.is {
        vel.x = MS;
    } else {
        vel.x = -MS;
    }
    if keyboard_input.pressed(KeyCode::Space) && on_ground {
        vel.y = 160.;
        *just_jumped = true;
        *t = time.elapsed();
    }
    if !on_ground && *just_jumped {
        *just_jumped = false;
        *inair = true;
    }
    const ROT_PER_MS: f32 = PI / 1090.;
    if !on_ground {
        let mut cube_t = cube_transform_q.single_mut().expect("no cube(");
        cube_t.rotate_z(ROT_PER_MS * time.delta().as_millis() as f32);
    }
}

fn fix_casters(
    mut casters: Query<(&mut ShapeCaster, &CastDir)>,
    parent_q: Query<&Transform, With<Cube>>,
) {
    let parent_transform = parent_q.single().expect("no cube(");
    let parent_angle = parent_transform.rotation.to_euler(EulerRot::XYZ).2;
    let reverse_rotation = Vec2::from_angle(-parent_angle);
    for (mut caster, cast_dir) in casters.iter_mut() {
        let world_goal;
        match cast_dir {
            CastDir::X => {
                world_goal = Vec2::X;
            },
            CastDir::Y => {
                world_goal = Vec2::Y;
            },
            CastDir::NEGX => {
                world_goal = Vec2::NEG_X;
            },
            CastDir::NEGY => {
                world_goal = Vec2::NEG_Y;
            },
        }

        let local_direction = world_goal.rotate(reverse_rotation);
        caster.direction = Dir2::new(local_direction).unwrap_or(Dir2::X);
        caster.shape_rotation = -parent_angle;
    }
}

fn tick_defeat(
    mut cmd: Commands,
    mut t: Local<f32>,
    time: Res<Time>,
    mut state: ResMut<NextState<AppState>>,
    mut screenshot: ResMut<LastScreenshot>,
    canvas: Res<camera::ViewportCanvas>,
){
    let dt = time.delta_secs().min(MAX_DT);
    *t += dt;
    if *t >= DEATH_DELAY {
        if screenshot.awaiting == false {
            cmd.spawn(bevy::render::view::screenshot::Screenshot::image(canvas.image.clone()))
                .observe(await_screenshot_and_translate(AppState::Defeat));
            screenshot.awaiting = true;
        }
        state.set(AppState::Defeat);
    }
}

fn tick_win(
    mut state: ResMut<NextState<AppState>>,
) {
    state.set(NEXT_STATE);
}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
    mut proj: Query<&mut Projection, With<WorldCamera>>
) {
    match &mut *proj.single_mut().expect("nocam") {
        Projection::Orthographic(proj) => {
            proj.scale = 0.8;
        },
        _ => {}
    }
    cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
    cmd.remove_resource::<PlayerEntity>();
    cmd.remove_resource::<IsLeft>();
}